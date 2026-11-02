#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![recursion_limit = "256"]

use freya::prelude::*;
use freya::radio::use_init_radio_station;
use aetherhaven_launcher::ipc::{self, Claim};
use aetherhaven_launcher::state::{AppChannel, AppState, LauncherInit};
use aetherhaven_launcher::{
    Actions, ConfirmLinkOverlay, DisableConfirmState, DisableDependentsOverlay,
    EssentialConfirmOverlay, EssentialGuardState, EventPump, LinkConfirmState, StartMaximizedState,
    cli, constants, events, microsoft_java, platform, router, theme, use_provide_actions,
    use_provide_disable_confirm, use_provide_essential_guard, use_provide_link_confirm,
    use_provide_start_maximized, use_settings_snapshot,
};
use std::cell::Cell;
use tokio::runtime::Builder;

struct AetherHavenApp {
    needs_location: bool,
    start_maximized: bool,
    boot_launch: Cell<Option<String>>,
}

impl App for AetherHavenApp {
    fn render(&self) -> impl IntoElement {
        let needs_location = self.needs_location;

        // Radio state is `!Send` so every writer runs on the UI thread `spawn_forever`
        let station = use_init_radio_station::<AppState, AppChannel>(move || AppState {
            launcher: LauncherInit {
                needs_location,
                ..LauncherInit::default()
            },
            ..AppState::default()
        });

        let boot_launch = self.boot_launch.take();

        let actions = use_hook(move || {
            let (signals_tx, signals_rx) = tokio::sync::mpsc::unbounded_channel();
            let (events_bus, events_rx) = aetherhaven_events::EventBus::channel();
            let actions = Actions::new(station, signals_tx, events_bus.clone());

            spawn_forever(
                EventPump {
                    events: events_rx,
                    signals: signals_rx,
                    station,
                }
                .run(),
            );

            if !needs_location {
                let startup = actions.clone();
                let rescue_bus = events_bus.clone();
                spawn_forever(async move {
                    match events::start_launcher(station, events_bus).await {
                        // Must follow startup `sync_bundles` needs the launcher handle and firing
                        // it early leaves `syncing_bundles` stuck disabling every launch button
                        Ok(()) => {
                            startup.sync_bundles();
                            Actions::check_sessions();
                            microsoft_java::spawn_auto_install();
                        }
                        Err(err) => {
                            events::report_startup_failure(&station, &err);
                            aetherhaven_launcher::updater::spawn_update_check(false, rescue_bus);
                        }
                    }
                });
            }

            if let Some(folder) = boot_launch {
                actions.request_launch_by_folder(folder);
            }

            actions
        });

        use_provide_actions(&actions);

        let link_confirm = use_state(|| None::<String>);
        use_provide_link_confirm(LinkConfirmState(link_confirm));

        let disable_confirm = use_state(|| None);
        use_provide_disable_confirm(DisableConfirmState(disable_confirm));

        let essential_guard = use_state(|| None);
        use_provide_essential_guard(EssentialGuardState(essential_guard));

        use_provide_start_maximized(StartMaximizedState(self.start_maximized));

        let mut palette_revision = use_state(|| 0u32);
        theme::colors::bind_revision(palette_revision);
        let appearance = use_settings_snapshot().settings;
        theme::colors::apply(appearance.theme, appearance.accent_color.as_deref());
        use_side_effect_with_deps(&(appearance.theme, appearance.accent_color), move |_| {
            *palette_revision.write() += 1;
        });

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(ConfirmLinkOverlay)
            .child(DisableDependentsOverlay)
            .child(EssentialConfirmOverlay)
            .child(router())
    }
}

fn main() {
    #[cfg(target_os = "linux")]
    unsafe {
        gtk_sys::gtk_disable_setlocale();
    }

    let cli = cli::parse();

    let mut builder = Builder::new_multi_thread();
    builder.enable_all().max_blocking_threads(64);

    // Debug builds emit unoptimized async code that can overflow the default 2MB stack
    #[cfg(debug_assertions)]
    builder.thread_stack_size(3 * 1024 * 1024);

    let rt = builder.build().unwrap();
    let _tokio_guard = rt.enter();

    let adopted = rt.block_on(aetherhaven_core::relocate::adopt_legacy_dir());

    let preview_mode = std::env::var("AETHERHAVEN_PREVIEW")
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes"))
        .unwrap_or(false);

    // no settings file is the sign of a fresh install, but not proof of one.
    // Preview must also bypass the first-run location gate; otherwise the UI
    // skips onboarding but the launcher task is never started.
    let never_set_up = aetherhaven_common::paths::settings_file()
        .map(|path| !path.exists())
        .unwrap_or(false);
    let has_database = aetherhaven_common::paths::database_file()
        .map(|path| path.exists())
        .unwrap_or(false);
    let was_damaged = aetherhaven_common::paths::damaged_settings_file()
        .map(|path| path.exists())
        .unwrap_or(false);
    let needs_location = !preview_mode && never_set_up && !has_database && !was_damaged;

    let mut unprotected = None;
    match rt.block_on(ipc::claim(&cli)) {
        Claim::Forwarded => return,
        Claim::Primary(listener) => ipc::listen(listener),
        Claim::Solo(reason) => unprotected = Some(reason),
    }

    let settings = rt.block_on(aetherhaven_core::settings::store::load_settings(None));

    aetherhaven_common::consent::init(settings.declined_tos);

    let abandoned_move = settings
        .data_dir
        .clone()
        .filter(|dir| aetherhaven_core::relocate::incomplete(dir));

    if let Some(dir) = settings.data_dir.clone()
        && abandoned_move.is_none()
    {
        aetherhaven_common::paths::set_data_dir(dir);
    }

    if settings.log_debug {
        aetherhaven_core::logger::init_debug()
    } else {
        aetherhaven_core::logger::init()
    }
    .expect("Failed to initialize logger");

    match adopted {
        Ok(Some(from)) => {
            tracing::info!(from = %from.display(), "moved out of the old launcher folder")
        }
        Ok(None) => {}
        Err(err) => tracing::error!("{err}"),
    }

    if let Some(reason) = unprotected {
        tracing::warn!("no single-instance endpoint, a second launcher can start: {reason}");
    }

    match aetherhaven_launcher::shortcut::launcher_exe()
        .and_then(|exe| aetherhaven_launcher::protocol::register(&exe))
    {
        Ok(()) => {}
        Err(err) => tracing::warn!(
            "could not register the {}:// handler: {err:#}",
            aetherhaven_launcher::protocol::SCHEME
        ),
    }

    if let Some(dir) = abandoned_move {
        tracing::error!(
            path = %dir.display(),
            "a move of the game data folder never finished; staying on the old folder"
        );
    }

    let _sentry_guard = aetherhaven_core::reporting::init(settings.crash_reporting);

    #[cfg(target_os = "macos")]
    aetherhaven_launcher::platform::macos::loop_memory_collector();

    theme::colors::apply(settings.theme, settings.accent_color.as_deref());

    let start_maximized = settings.start_maximized;
    let show_tray_icon = settings.show_tray_icon;

    let window_config = WindowConfig::new_app(AetherHavenApp {
        needs_location,
        start_maximized,
        boot_launch: Cell::new(cli.launch),
    })
    .with_title(constants::WINDOW_TITLE)
    .with_app_id(constants::WINDOW_APP_ID)
    .with_icon(LaunchConfig::window_icon(include_bytes!(
        "../icons/128x128.png"
    )))
    .with_size(1200., 800.)
    .with_min_size(800., 600.)
    .with_transparency(false)
    .with_background(Color::TRANSPARENT)
    // A half-copied data folder is unrecoverable, so the window refuses to
    // close while one is being moved; the move screen says as much
    .with_on_close(|_, _| {
        if aetherhaven_core::relocate::in_progress() {
            tracing::warn!("close request ignored, the data folder is still being moved");
        } else {
            ipc::send(ipc::IpcCommand::Close);
        }
        CloseDecision::KeepOpen
    });

    #[cfg(target_os = "macos")]
    let window_config =
        window_config
            .with_decorations(true)
            .with_window_attributes(move |attrs, _| {
                use freya::winit::platform::macos::WindowAttributesExtMacOS;
                attrs
                    .with_title_hidden(true)
                    .with_titlebar_transparent(true)
                    .with_titlebar_buttons_hidden(true)
                    .with_fullsize_content_view(true)
                    .with_has_shadow(true)
                    .with_maximized(start_maximized)
            });

    #[cfg(not(target_os = "macos"))]
    let window_config = window_config
        .with_decorations(false)
        .with_window_attributes(move |attrs, _| attrs.with_maximized(start_maximized));

    let mut launch_config = LaunchConfig::new()
        .with_window(window_config)
        .with_gpu_resource_cache_limit(
            std::env::var("AETHERHAVEN_GPU_CACHE_BYTES")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(128 * 1024 * 1024),
        )
        .with_plugin(freya::borderless::BorderlessPlugin::new())
        .with_plugin(freya::metrics::MetricsPlugin::default())
        .with_default_font(theme::DEFAULT_FONT);

    #[cfg(target_os = "windows")]
    {
        launch_config = launch_config.with_plugin(platform::frame::FramePlugin);
    }

    if show_tray_icon && platform::tray::available() {
        launch_config = launch_config.with_tray(platform::tray::build, platform::tray::handle);
    }

    for (font, bytes) in theme::load_fonts() {
        launch_config = launch_config.with_font(font, bytes);
    }

    launch(launch_config);
}

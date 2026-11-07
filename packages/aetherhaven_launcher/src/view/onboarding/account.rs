use freya::prelude::*;
use freya::query::{MutationStateData, UseMutation};
use aetherhaven_auth::MinecraftAccount;

use crate::components::{
    AssetImage, Avatar, Button, Icon, IconType, TextInput, use_microsoft_login,
};
use crate::hooks::{
    try_default_account, use_add_offline_account, use_current_account, AddOfflineAccountKeys,
};
use crate::routes::Route;
use crate::theme::colors;
use crate::view::onboarding::{
    onboarding_nav, onboarding_page, step_heading,
};

#[derive(PartialEq)]
pub struct OnboardingAccount;

impl Component for OnboardingAccount {
    fn render(&self) -> impl IntoElement {
        let account_query = use_current_account();
        let add_offline = use_add_offline_account();
        let microsoft = use_microsoft_login();
        let username = use_state(|| String::from("Player"));
        let mut submitting = use_state(|| false);

        let account = try_default_account(&account_query);
        let has_account = account.is_some();
        let error = mutation_err_text(&add_offline);
        let pending = *submitting.read();

        let on_add = {
            let name = username.read().trim().to_string();
            let add_offline = add_offline.clone();
            move |_| {
                if name.is_empty() || pending {
                    return;
                }
                submitting.set(true);
                add_offline.mutate(AddOfflineAccountKeys { username: name.clone() });
            }
        };

        use_side_effect(move || {
            if !pending {
                return;
            }
            if matches!(&*add_offline.read().state(), MutationStateData::Settled { .. }) {
                submitting.set(false);
            }
        });

        let microsoft_login = Button::new()
            .primary()
            .large()
            .enabled(!microsoft.pending)
            .on_press({
                let microsoft = microsoft.clone();
                move |_| microsoft.start()
            })
            .child(Icon::new(IconType::Globe01).size(16.))
            .text(if microsoft.pending {
                "Signing in..."
            } else {
                "Login with Microsoft"
            });

        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Player profile",
                "Use an official Microsoft account or continue with a local offline profile.",
            ))
            .child(
                rect()
                    .vertical()
                    .width(Size::fill())
                    .spacing(8.)
                    .child(
                        label()
                            .text("OFFICIAL ACCOUNT")
                            .font_size(11.)
                            .font_weight(FontWeight::MEDIUM)
                            .color(colors::fg_secondary()),
                    )
                    .child(
                        label()
                            .text("Use Microsoft login for premium and official servers.")
                            .font_size(12.)
                            .color(colors::fg_secondary()),
                    )
                    .child(microsoft_login)
                    .maybe_child(microsoft.error.clone().map(|message| {
                        label().text(message).font_size(12.).color(colors::danger())
                    })),
            )
            .child(
                label()
                    .text("OR USE OFFLINE")
                    .font_size(11.)
                    .font_weight(FontWeight::MEDIUM)
                    .color(colors::fg_secondary()),
            )
            .child(match &account {
                Some(account) => account_preview(account).into_element(),
                None => offline_profile_card(
                    username,
                    pending,
                    error,
                    on_add,
                )
                .into_element(),
            })
            .into_element();

        let page = onboarding_page(
            AssetImage::new("aether_logo_HD.png", 240.),
            content,
            onboarding_nav(
                Some(Route::OnboardingLanguage {}),
                Route::OnboardingBundles {},
                has_account,
            ),
        );

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(page)
            .maybe_child(microsoft.popup())
    }
}

fn account_preview(account: &MinecraftAccount) -> impl IntoElement {
    rect()
        .horizontal()
        .width(Size::fill())
        .spacing(24.)
        .child(
            rect()
                .horizontal()
                .spacing(12.)
                .cross_align(Alignment::Center)
                .child(
                    Avatar::new(account.id.to_string())
                        .width(Size::px(48.))
                        .height(Size::px(48.)),
                )
                .child(
                    rect()
                        .vertical()
                        .spacing(4.)
                        .child(
                            label()
                                .text(account.username.clone())
                                .font_size(16.)
                                .font_weight(FontWeight::SEMI_BOLD)
                                .color(colors::fg_primary()),
                        )
                        .child(
                            label()
                                .text("Offline profile")
                                .font_size(12.)
                                .color(colors::fg_secondary()),
                        ),
                ),
        )
        .into_element()
}

fn offline_profile_card(
    username: State<String>,
    pending: bool,
    error: Option<String>,
    on_add: impl FnMut(Event<PressEventData>) + 'static,
) -> impl IntoElement {
    rect()
        .vertical()
        .spacing(12.)
        .width(Size::fill())
        .child(
            label()
                .text("Offline profile")
                .font_size(16.)
                .font_weight(FontWeight::SEMI_BOLD)
                .color(colors::fg_primary()),
        )
        .child(
            label()
                .text("This profile works with Singleplayer and servers that allow offline-mode players. Official Minecraft servers still require Microsoft authentication.")
                .font_size(12.)
                .color(colors::fg_secondary()),
        )
        .child(TextInput::new(username).placeholder("Username"))
        .child(
            Button::new()
                .primary()
                .large()
                .enabled(!pending)
                .on_press(on_add)
                .child(Icon::new(IconType::Plus).size(16.))
                .text(if pending { "Creating profile..." } else { "Continue offline" }),
        )
        .maybe_child(error.map(|message| {
            label().text(message).font_size(12.).color(colors::danger())
        }))
}

fn mutation_err_text<T>(mutation: &UseMutation<T>) -> Option<String>
where
    T: freya::query::MutationCapability,
    T::Err: std::fmt::Display,
{
    match &*mutation.read().state() {
        MutationStateData::Settled { res: Err(error), .. } => Some(error.to_string()),
        MutationStateData::Loading {
            res: Some(Err(error)),
        } => Some(error.to_string()),
        _ => None,
    }
}

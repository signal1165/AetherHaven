use std::str::FromStr;

use aetherhaven_db::dao::{bundle as bundle_dao, cluster as cluster_dao};

use crate::LauncherResult;
use crate::state::LauncherState;
use aetherhaven_common::domain::GameLoader;
use aetherhaven_common::version::format_mc_version;

use aetherhaven_cluster::Cluster;
use aetherhaven_cluster::CreateClusterOptions;

const DEFAULT_PROFILES: [(&str, GameLoader, &str); 5] = [
    ("1.8.9", GameLoader::Forge, "Classic PvP"),
    ("1.16.5", GameLoader::Forge, "Nether Edition"),
    ("1.21.11", GameLoader::Fabric, "Trials Edition"),
    ("26.2", GameLoader::Fabric, "Slime Edition"),
    ("26.3", GameLoader::Fabric, "AetherHaven Edition"),
];

#[tracing::instrument(skip(state))]
pub async fn ensure_from_bundles(state: &LauncherState) -> LauncherResult<Vec<Cluster>> {
    let groups = bundle_dao::list_distinct_version_loaders(&state.services.db).await?;
    let mut created = Vec::new();

    for group in groups {
        let Some(loader) = GameLoader::from_repr(group.mc_loader as u8) else {
            tracing::warn!(
                mc_version = %group.mc_version,
                mc_loader = group.mc_loader,
                "skipping bundle group with unknown loader"
            );
            continue;
        };

        if cluster_dao::find_by_version_loader(
            &state.services.db,
            &group.mc_version,
            group.mc_loader,
        )
        .await?
        .is_some()
        {
            continue;
        }

        let mc_version = group.mc_version.clone();
        let name = format!("{mc_version} {loader}");
        let global = state.settings.read().global_game_settings.clone();
        match state
            .clusters
            .create_provisioned(
                &global,
                CreateClusterOptions::new(name, mc_version.clone(), loader),
            )
            .await
        {
            Ok(Some(cluster)) => {
                tracing::info!(
                    cluster_id = cluster.id,
                    mc_version = %cluster.mc_version,
                    loader = %cluster.mc_loader,
                    "created cluster from bundle catalog"
                );
                created.push(cluster);
            }
            Ok(None) => {}
            Err(err) => {
                tracing::warn!(
                    mc_version = %mc_version,
                    loader = %loader,
                    error = %err,
                    "failed to create cluster for bundle group"
                );
            }
        }
    }

    Ok(created)
}

#[tracing::instrument(skip(state))]
pub async fn ensure_from_versions(state: &LauncherState) -> LauncherResult<Vec<Cluster>> {
    let metadata = state
        .versions
        .metadata(&state.services.requester.config().meta_url_base)
        .await;
    let mut created = Vec::new();

    for entry in metadata {
        let Some(minor) = entry.minor_version else {
            continue;
        };
        let Some(loader_str) = entry.loader.as_deref() else {
            continue;
        };
        let Ok(loader) = GameLoader::from_str(loader_str) else {
            tracing::warn!(
                major = entry.major_version,
                minor,
                loader = loader_str,
                "skipping versions entry with unknown loader"
            );
            continue;
        };

        let mc_version = format_mc_version(entry.major_version, minor, entry.patch_version);

        if cluster_dao::find_by_version_loader(&state.services.db, &mc_version, loader as i64)
            .await?
            .is_some()
        {
            continue;
        }

        let name = format!("{mc_version} {loader}");
        let global = state.settings.read().global_game_settings.clone();
        match state
            .clusters
            .create_provisioned(
                &global,
                CreateClusterOptions::new(name, mc_version.clone(), loader),
            )
            .await
        {
            Ok(Some(cluster)) => {
                tracing::info!(
                    cluster_id = cluster.id,
                    mc_version = %cluster.mc_version,
                    loader = %cluster.mc_loader,
                    "created cluster from versions manifest"
                );
                created.push(cluster);
            }
            Ok(None) => {}
            Err(err) => {
                tracing::warn!(
                    mc_version = %mc_version,
                    loader = %loader,
                    error = %err,
                    "failed to create cluster for versions entry"
                );
            }
        }
    }

    Ok(created)
}

/// Ensures the launcher has a useful first-run catalog even when the remote
/// versions manifest is unavailable. These are profiles, not bundled game
/// files: the normal prepare step still downloads the selected Minecraft and
/// loader artifacts when the player launches one.
#[tracing::instrument(skip(state))]
pub async fn ensure_from_default_profiles(state: &LauncherState) -> LauncherResult<Vec<Cluster>> {
    let mut created = Vec::new();
    let global = state.settings.read().global_game_settings.clone();

    for &(mc_version, loader, description) in &DEFAULT_PROFILES {
        if cluster_dao::find_by_version_loader(&state.services.db, mc_version, loader as i64)
            .await?
            .is_some()
        {
            continue;
        }

        let name = format!("{mc_version} {loader}");
        match state
            .clusters
            .create_provisioned(
                &global,
                CreateClusterOptions::new(name, mc_version, loader)
                    .description(description)
                    .tags(vec!["AetherHaven".to_string(), "Default".to_string()]),
            )
            .await
        {
            Ok(Some(cluster)) => {
                tracing::info!(
                    cluster_id = cluster.id,
                    mc_version = %cluster.mc_version,
                    loader = %cluster.mc_loader,
                    "created default AetherHaven profile"
                );
                created.push(cluster);
            }
            Ok(None) => {}
            Err(err) => {
                tracing::warn!(
                    mc_version,
                    loader = %loader,
                    error = %err,
                    "failed to create default AetherHaven profile"
                );
            }
        }
    }

    Ok(created)
}

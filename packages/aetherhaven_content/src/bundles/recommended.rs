use std::collections::HashSet;

use aetherhaven_common::domain::{GameLoader, ProviderId};
use aetherhaven_events::GroupedProgressChild;

use crate::ctx::ContentCtx;
use crate::error::ContentResult;
use crate::packages::dependencies::resolve_required;
use crate::packages::provider::PackageProvider;
use crate::packages::store::PackageStore;

/// Project IDs are resolved at install time, so every profile receives the
/// newest compatible release for its own Minecraft version and loader.
fn profile_projects(version: &str, loader: GameLoader) -> &'static [&'static str] {
    match loader {
        GameLoader::Fabric if version == "1.21.11" || version == "26.2" || version == "26.3" => &[
            "fabric-api",
            "sodium",
            "lithium",
            "ferritecore",
            "immediatelyfast",
            "entityculling",
            "modernfix",
        ],
        GameLoader::Forge if version == "1.8.9" => &["patcher", "foamfix-optimization"],
        GameLoader::Forge if version == "1.16.5" => &[
            "ferritecore",
            "modernfix",
            "embeddium",
            "entityculling",
        ],
        _ => &[],
    }
}

/// Installs the AetherHaven-managed performance profile for a default cluster.
/// Compatibility is resolved by the provider, never by blindly downloading a
/// fixed JAR, and required dependencies are installed recursively.
#[tracing::instrument(skip(ctx), fields(cluster_id))]
pub async fn install_recommended_profile(
    cluster_id: i64,
    ctx: &ContentCtx,
) -> ContentResult<Vec<String>> {
    let cluster = PackageStore::get_cluster(cluster_id, ctx).await?;
    let projects = profile_projects(&cluster.mc_version, cluster.mc_loader);
    if projects.is_empty() {
        return Ok(Vec::new());
    }

    let provider = ctx.providers.get(ProviderId::Modrinth)?;
    let mut installed = Vec::new();
    let mut seen = HashSet::new();

    for project_id in projects {
        let project = match provider.get_project(project_id, ctx).await {
            Ok(project) => project,
            Err(error) => {
                tracing::warn!(project_id, %error, "recommended project is unavailable");
                continue;
            }
        };

        let versions = provider
            .list_versions(
                project_id,
                Some(&cluster.mc_version),
                Some(cluster.mc_loader),
                0,
                50,
                ctx,
            )
            .await?;
        let Some(summary) = versions.items.into_iter().find(|version| {
            version
                .loaders
                .iter()
                .any(|loader| loader.compatible_with(cluster.mc_loader))
        }) else {
            tracing::info!(project_id, version = %cluster.mc_version, loader = %cluster.mc_loader, "no compatible recommended release");
            continue;
        };

        let version = provider.get_version(project_id, &summary.version_id, ctx).await?;
        if !seen.insert(version.project_id.clone()) {
            continue;
        }

        PackageStore::install_to_cluster(
            ProviderId::Modrinth,
            &project,
            &version,
            cluster_id,
            false,
            false,
            None::<&GroupedProgressChild>,
            ctx,
        )
        .await?;
        installed.push(project.name.clone());

        let dependencies = resolve_required(ProviderId::Modrinth, &version, cluster_id, ctx).await?;
        for dependency in dependencies.install {
            if seen.insert(dependency.version.project_id.clone()) {
                PackageStore::install_to_cluster(
                    ProviderId::Modrinth,
                    &dependency.project,
                    &dependency.version,
                    cluster_id,
                    false,
                    false,
                    None::<&GroupedProgressChild>,
                    ctx,
                )
                .await?;
                installed.push(dependency.project.name);
            }
        }

        if !dependencies.unresolved.is_empty() {
            tracing::warn!(project_id, unresolved = ?dependencies.unresolved, "recommended profile has unresolved dependencies");
        }
    }

    Ok(installed)
}

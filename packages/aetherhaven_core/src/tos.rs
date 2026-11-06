use serde::{Deserialize, Serialize};

use crate::LauncherResult;
use aetherhaven_common::constants;
use aetherhaven_common::paths;
use aetherhaven_net::RequestClient;
use aetherhaven_net::{EtagPolicy, fetch_cached};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TermsDocument {
    pub version: u32,
    #[serde(default)]
    pub privacy_version: Option<u32>,
    #[serde(default)]
    pub updated_at: Option<String>,
    pub terms: String,
    #[serde(default)]
    pub privacy: Option<String>,
    #[serde(default)]
    pub terms_url: Option<String>,
    #[serde(default)]
    pub privacy_url: Option<String>,
}

impl TermsDocument {
    pub fn terms_url(&self) -> &str {
        self.terms_url
            .as_deref()
            .filter(|url| !url.trim().is_empty())
            .unwrap_or(constants::TOS_URL)
    }

    pub fn privacy_url(&self) -> &str {
        self.privacy_url
            .as_deref()
            .filter(|url| !url.trim().is_empty())
            .unwrap_or(constants::PRIVACY_URL)
    }

    pub fn privacy_version(&self) -> u32 {
        self.privacy_version.unwrap_or(self.version)
    }

    pub fn privacy_body(&self) -> Option<&str> {
        self.privacy
            .as_deref()
            .filter(|body| !body.trim().is_empty())
    }
}

#[tracing::instrument(level = "debug", skip(net))]
pub async fn fetch_terms(net: &RequestClient) -> LauncherResult<TermsDocument> {
    let url = format!("{}/aetherhaven/tos.json", net.config().meta_url_base);
    let cache_path = paths::caches_dir()?.join("TERMS.json");

    if let Ok(Some(fetched)) = fetch_cached(net, &url, &cache_path, EtagPolicy::CommitNow).await {
        if let Ok(document) = fetched.json() {
            return Ok(document);
        }
    }

    Ok(local_terms_document())
}

fn local_terms_document() -> TermsDocument {
    TermsDocument {
        version: 1,
        privacy_version: Some(1),
        updated_at: Some("2026-11-06".to_string()),
        terms: r#"# AetherHaven Terms of Service

**Last updated: 6 November 2026**

AetherHaven is an independent, open-source Minecraft launcher. By selecting **Accept**, you agree to use the launcher responsibly and to follow the rules of the servers you join.

## Open-source software

AetherHaven is distributed under the license included with the project. Third-party libraries, Minecraft loaders, and mods remain subject to their own licenses and attribution requirements.

## Accounts and offline mode

AetherHaven supports local offline profiles for single-player and servers that permit offline-mode players. An offline profile does not prove ownership of Minecraft and cannot be used to access servers that require official authentication.

Official Minecraft authentication is optional in the launcher. If you use it, authentication is handled by Microsoft and the Minecraft services; AetherHaven does not ask for or store your Microsoft password.

## Content and downloads

Mods and other content may be downloaded from their respective public providers, including Modrinth. Each item remains the property of its authors and is subject to its own license. AetherHaven does not guarantee that third-party content will always be available or compatible with every game version.

## Disclaimer

AetherHaven is provided on an “as is” basis. You are responsible for backups of worlds, settings, and downloaded content. AetherHaven is not an official Minecraft product and is not approved by or associated with Mojang or Microsoft.

## Contact

Project discussions and issue reports are available at the AetherHaven GitHub repository."#
            .to_string(),
        privacy: Some(
            r#"# AetherHaven Privacy Policy

**Last updated: 6 November 2026**

## Data stored locally

AetherHaven stores launcher settings, local profiles, selected versions, logs, and downloaded content on your computer so the launcher can work between sessions. Offline profile data stays local to the launcher.

## Microsoft sign-in

Microsoft sign-in is optional. When you choose it, the authentication flow is opened through Microsoft’s services. AetherHaven stores the tokens needed to keep your authorized session, but does not store your Microsoft password.

## Network requests

The launcher may contact Minecraft services, Modrinth, CurseForge, GitHub, and Java distribution providers to download game files, loaders, mods, updates, or project information. Optional service integrations are disabled unless explicitly enabled by the user.

## Diagnostics

Crash and diagnostic reporting is limited to what is enabled in the launcher settings. You can disable optional diagnostics before accepting this policy or later in Settings.

## Changes

If this policy changes, AetherHaven will show the updated version during startup and ask you to review it again.

## Contact

For privacy questions, open an issue or discussion in the AetherHaven GitHub repository."#
                .to_string(),
        ),
        terms_url: Some(constants::TOS_URL.to_string()),
        privacy_url: Some(constants::PRIVACY_URL.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_document() {
        let raw = br###"{"version":2,"terms":"## Terms\n\nBe nice."}"###;
        let document: TermsDocument = serde_json::from_slice(raw).unwrap();

        assert_eq!(document.version, 2);
        assert_eq!(document.terms, "## Terms\n\nBe nice.");
        assert_eq!(document.terms_url(), constants::TOS_URL);
        assert_eq!(document.privacy_url(), constants::PRIVACY_URL);
    }

    #[test]
    fn privacy_version_falls_back_to_terms_version() {
        let raw = br#"{"version":4,"terms":"x"}"#;
        let document: TermsDocument = serde_json::from_slice(raw).unwrap();

        assert_eq!(document.privacy_version(), 4);
        assert_eq!(document.privacy_body(), None);
    }

    #[test]
    fn privacy_version_is_independent_when_published() {
        let raw = br###"{"version":4,"privacy_version":2,"terms":"x","privacy":"## Privacy"}"###;
        let document: TermsDocument = serde_json::from_slice(raw).unwrap();

        assert_eq!(document.version, 4);
        assert_eq!(document.privacy_version(), 2);
        assert_eq!(document.privacy_body(), Some("## Privacy"));
    }

    #[test]
    fn blank_privacy_body_is_treated_as_absent() {
        let raw = br#"{"version":1,"terms":"x","privacy":"   "}"#;
        let document: TermsDocument = serde_json::from_slice(raw).unwrap();

        assert_eq!(document.privacy_body(), None);
    }

    #[test]
    fn document_urls_override_constants() {
        let raw = br#"{"version":1,"terms":"x","terms_url":"https://example.com/tos","privacy_url":"  "}"#;
        let document: TermsDocument = serde_json::from_slice(raw).unwrap();

        assert_eq!(document.terms_url(), "https://example.com/tos");
        assert_eq!(document.privacy_url(), constants::PRIVACY_URL);
    }
}

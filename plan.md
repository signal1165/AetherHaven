# AetherHaven Launcher Theme Plan

## Product direction
AetherHaven is a branded Minecraft launcher fork built on OneLauncher/AetherHaven 2.9.0. The existing Rust/Freya launcher functionality remains the product foundation; this pass changes visible identity and the Home experience without replacing account, version, mod, Java, or launch systems.

## Design
- **Design movement:** Sky-island glassmorphism: a black atmospheric layer, translucent glass panels, and a warm orange crown as the ownership cue.
- **Core principles:** readable over artwork; orange only for actions and identity; soft glass surfaces; Minecraft-native but not pixel-art-heavy.
- **Color philosophy:** near-black keeps the launcher focused and legible over the supplied sky-island background; Aether orange signals the active/launch state; pale cyan-white text preserves contrast.
- **Layout paradigm:** retain the proven OneLauncher shell: top navigation, full-bleed home art, left-side version selection and a prominent launch action.
- **Signature elements:** orange crown and wordmark, black atmospheric backdrop, glass navigation and cards with thin warm borders on selected states.
- **Interaction philosophy:** selected versions and primary actions are obvious at a glance; hover states brighten rather than jump or flash.
- **Animation:** preserve the existing fade/parallax system; keep transitions short and calm so background motion never competes with Launch.
- **Typography:** retain the existing UI font system for localization and platform stability; use bold display weight for version title and medium weight for navigation.
- **Brand essence:** a welcoming, polished Minecraft client launcher for AetherHaven players; serene, capable, adventurous.
- **Brand voice:** short and confident. Example: “Launch into AetherHaven.” / “Your worlds, ready when you are.”
- **Wordmark & logo:** a gold orbital halo paired with the two-line Aether/Haven wordmark; the SVG remains vector and recolorable where the host viewer permits.
- **Signature brand color:** Aether Orange `#FAA81E`.

## Implementation
- Replace visible launcher logo asset with AetherHaven SVG.
- Update dark/AMOLED palettes to the blue/gold direction while leaving light themes usable.
- Update window/package metadata to AetherHaven.
- Keep `backgrounds/CavesAndCliffs.jpg` from the 2.9.0 source repository as the initial Home background.
- Preserve GPL-3.0-only, upstream attribution, and a prominent modified-version notice before redistribution.

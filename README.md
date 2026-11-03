<div align="center">

<img src=".github/media/RepoBanner.png" alt="Repository Banner" />

# AetherHaven  |  AetherHaven Launcher
The monorepo containing the code for AetherHaven Launcher, AetherHaven, and their core backend.

AetherHaven is a Minecraft client featuring fully 100% open-source components, offering many packaged and pre-configured mods in one click.
AetherHaven Launcher is a WIP Minecraft launcher giving power-users the greatest customization whilst featuring a clean UI.

</div>

## Installing

You can install the latest release of AetherHaven from our website: [https://github.com/signal1165/AetherHaven](https://github.com/signal1165/AetherHaven)
as well as our [GitHub releases](https://github.com/signal1165/AetherHaven/releases/latest).

| Windows (x86_64) | macOS (Intel & Apple Silicon) | Linux (x86_64)                                          |
|------------------|-------------------------------|---------------------------------------------------------|
| Installer 🔄      | DMG 🔄                        | AppImage 🔄                                             |
|                  | App Bundle 🔄                 | DEB                                                     |
|                  |                               | RPM                                                     |
|                  |                               | [AUR](https://aur.archlinux.org/packages/aetherhaven-bin) |

> 🔄 = Has support for autoupdating built-in


## Contributing

We welcome contributions! Please read our [contributing guidelines](CONTRIBUTING.md) before getting started.


### Requirements

The project targets **Rust 1.97** or later. You can install Rust via [rustup](https://rustup.rs/).


### Building & Running

```sh
# Run the app
cargo run -p aetherhaven_launcher

# Build a release binary
cargo build -p aetherhaven_launcher --release
```


### Packaging / Releasing

Installers are produced with [**cargo-packager**](https://github.com/crabnebula-dev/cargo-packager)
(the standalone bundler spun out of the Tauri bundler). Config lives in
[`packages/aetherhaven_launcher/Cargo.toml`](./packages/aetherhaven_launcher/Cargo.toml) under
`[package.metadata.packager]`.

```sh
cargo install cargo-packager --locked

# Build the binary, then bundle it for the current OS:
cargo build --release -p aetherhaven_launcher
cargo packager --release -p aetherhaven_launcher --formats <targets>
#   Windows: nsis      macOS: app,dmg      Linux: deb,appimage
```


### Versioning

The workspace shares a single version, defined in the root [`Cargo.toml`](./Cargo.toml) under `[workspace.package]`.

Versions and release notes come from [Knope](https://knope.tech) change files in [`.changeset/`](./.changeset).
Add one per user-facing change with `knope document-change` (installed via `cargo install knope`). The
`AetherHaven Release Build` workflow consumes them, bumps the version, writes [`CHANGELOG.md`](./CHANGELOG.md),
and uses the new entry as the GitHub release body, which is what the launcher's changelog page shows.


## Code signing

This program uses free code signing provided by [SignPath.io](https://signpath.io?utm_source=foundation&utm_medium=github&utm_campaign=0install), and a certificate by the [SignPath Foundation](https://signpath.org?utm_source=foundation&utm_medium=github&utm_campaign=0install). We thank them very much to their contributions to OSS software!

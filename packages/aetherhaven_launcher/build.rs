fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("icons/icon.ico");

        let version = env!("CARGO_PKG_VERSION");
        res.set("CompanyName", "AetherHaven Contributors");
        res.set("ProductName", "AetherHaven");
        res.set("FileDescription", "AetherHaven");
        res.set("LegalCopyright", "© 2026 AetherHaven Contributors");
        res.set("OriginalFilename", "aetherhaven_launcher.exe");
        res.set("InternalName", "aetherhaven_launcher");
        res.set("FileVersion", version);
        res.set("ProductVersion", version);

        res.compile()
            .expect("failed to embed Windows icon resource");
    }
}

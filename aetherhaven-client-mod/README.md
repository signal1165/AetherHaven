# AetherHaven Client Mod

این پروژه یک client-side Fabric mod برای Minecraft 1.21.1 است و شامل منوی داخل بازی، HUD نمایش FPS، حالت Performance و چند toggle برای client HUD می‌شود.

## کنترل‌ها

- `Right Shift`: بازکردن منوی AetherHaven
- `FPS HUD`: نمایش FPS و نام AetherHaven
- `Performance Mode`: پرچم تنظیمات performance برای توسعه‌ی بعدی
- `Menu Blur`: تنظیم ظاهری منو
- `Keystrokes HUD`: جایگاه آماده برای HUD کلیدها

## Build

این پروژه به Java 21 و اینترنت برای دریافت Fabric Loom، Minecraft mappings و Fabric API نیاز دارد:

```bash
gradle build
```

فایل خروجی در این مسیر قرار می‌گیرد:

```text
build/libs/aetherhaven-client-mod-0.1.0.jar
```

آن فایل را در پوشه‌ی `mods` پروفایل Fabric نسخه‌ی 1.21.1 قرار بده. Fabric API نیز باید در همان profile نصب باشد.

## محدودیت نسخه‌ها

این JAR برای Minecraft 1.21.1 ساخته شده است. برای 1.8.9، 1.16.5 و شاخه‌های 26.x باید buildهای جدا با mappings و loader سازگار تولید شود؛ یک JAR واحد نمی‌تواند همه‌ی این نسخه‌ها را پوشش دهد.

این مود فقط قابلیت‌های client-side مجاز و قابل‌کنترل ارائه می‌دهد و احراز هویت Minecraft یا محدودیت‌های سرور را دور نمی‌زند.

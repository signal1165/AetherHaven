package com.aetherhaven.client;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.keybinding.v1.KeyBindingHelper;
import net.fabricmc.fabric.api.client.rendering.v1.HudRenderCallback;
import net.minecraft.client.MinecraftClient;
import net.minecraft.client.gui.DrawContext;
import net.minecraft.client.option.KeyBinding;
import net.minecraft.client.util.InputUtil;
import net.minecraft.client.network.ClientPlayerEntity;
import net.minecraft.text.Text;
import org.lwjgl.glfw.GLFW;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.time.Instant;

public final class AetherHavenClient implements ClientModInitializer {
    public static final String BRAND = "AetherHaven";
    public static final AetherHavenConfig CONFIG = AetherHavenConfig.load();
    private static KeyBinding menuKey;
    private static KeyBinding sprintKey;
    private static boolean sprintLatched;

    @Override
    public void onInitializeClient() {
        menuKey = KeyBindingHelper.registerKeyBinding(new KeyBinding(
                "key.aetherhaven.menu", InputUtil.Type.KEYSYM, GLFW.GLFW_KEY_RIGHT_SHIFT,
                "category.aetherhaven"));
        sprintKey = KeyBindingHelper.registerKeyBinding(new KeyBinding(
                "key.aetherhaven.sprint_toggle", InputUtil.Type.KEYSYM, GLFW.GLFW_KEY_G,
                "category.aetherhaven"));
        installCrashGuard();

        ClientTickEvents.END_CLIENT_TICK.register(client -> {
            while (menuKey.wasPressed()) {
                if (client.currentScreen == null) client.setScreen(new AetherHavenScreen());
            }
            while (sprintKey.wasPressed()) {
                sprintLatched = !sprintLatched;
                CONFIG.sprintToggle = sprintLatched;
                CONFIG.save();
            }
            ClientPlayerEntity player = client.player;
            if (player != null && CONFIG.sprintToggle && !player.isSneaking() && player.forwardSpeed > 0.0f) {
                player.setSprinting(true);
            }
        });

        HudRenderCallback.EVENT.register((context, tickCounter) -> {
            MinecraftClient client = MinecraftClient.getInstance();
            if (client.player != null && !client.options.hudHidden) renderHud(context, client);
        });
    }

    private static void renderHud(DrawContext context, MinecraftClient client) {
        int x = 10;
        int y = 10;
        int accent = 0xFFFFAA22;
        int panel = 0xB8101218;
        int line = 0;
        context.fill(x - 6, y - 5, x + 146, y + 20, panel);
        context.fill(x - 6, y - 5, x - 3, y + 20, accent);
        if (CONFIG.fpsHud) {
            context.drawTextWithShadow(client.textRenderer, Text.literal(BRAND), x, y + line, accent);
            line += 12;
            context.drawTextWithShadow(client.textRenderer, Text.literal("FPS  " + client.getCurrentFps()), x, y + line, 0xFFFFFFFF);
            line += 12;
        }
        if (CONFIG.coordinatesHud && client.player != null) {
            String coords = CONFIG.privacyMode ? "XYZ  hidden" : String.format("XYZ  %d %d %d",
                    client.player.getBlockX(), client.player.getBlockY(), client.player.getBlockZ());
            context.drawTextWithShadow(client.textRenderer, Text.literal(coords), x, y + line, 0xFFFFFFFF);
            line += 12;
        }
        if (CONFIG.keystrokesHud) {
            String keys = client.options.forwardKey.isPressed() ? "W" : "·";
            keys += client.options.leftKey.isPressed() ? " A" : " ·";
            keys += client.options.backKey.isPressed() ? " S" : " ·";
            keys += client.options.rightKey.isPressed() ? " D" : " ·";
            context.drawTextWithShadow(client.textRenderer, Text.literal(keys), x, y + line, accent);
        }
    }

    private static void installCrashGuard() {
        Thread.setDefaultUncaughtExceptionHandler((thread, error) -> {
            try {
                Path log = FabricLoaderBridge.configDir().resolve("aetherhaven-crash.log");
                Files.writeString(log, Instant.now() + " [" + thread.getName() + "]\n" + error + "\n",
                        StandardOpenOption.CREATE, StandardOpenOption.APPEND);
            } catch (Exception ignored) {
                // Crash reporting must never mask the original crash.
            }
    });
    }

    public static void saveConfig() { CONFIG.save(); }

    private static final class FabricLoaderBridge {
        private static Path configDir() {
            return net.fabricmc.loader.api.FabricLoader.getInstance().getConfigDir();
        }
    }
}

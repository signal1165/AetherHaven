package com.aetherhaven.client;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.keybinding.v1.KeyBindingHelper;
import net.fabricmc.fabric.api.client.rendering.v1.HudRenderCallback;
import net.minecraft.client.MinecraftClient;
import net.minecraft.client.option.KeyBinding;
import net.minecraft.client.util.InputUtil;
import net.minecraft.client.gui.DrawContext;
import net.minecraft.text.Text;
import org.lwjgl.glfw.GLFW;

public final class AetherHavenClient implements ClientModInitializer {
    public static final String BRAND = "AetherHaven";
    public static final AetherHavenConfig CONFIG = AetherHavenConfig.load();
    private static KeyBinding menuKey;

    @Override
    public void onInitializeClient() {
        menuKey = KeyBindingHelper.registerKeyBinding(new KeyBinding(
                "key.aetherhaven.menu",
                InputUtil.Type.KEYSYM,
                GLFW.GLFW_KEY_RIGHT_SHIFT,
                "category.aetherhaven"
        ));

        ClientTickEvents.END_CLIENT_TICK.register(client -> {
            while (menuKey.wasPressed()) {
                if (client.currentScreen == null) {
                    client.setScreen(new AetherHavenScreen());
                }
            }
        });

        HudRenderCallback.EVENT.register((context, tickCounter) -> {
            MinecraftClient client = MinecraftClient.getInstance();
            if (CONFIG.fpsHud && client.player != null && client.options.hudHidden == false) {
                renderHud(context, client);
            }
        });
    }

    private static void renderHud(DrawContext context, MinecraftClient client) {
        int fps = client.getCurrentFps();
        int x = 10;
        int y = 10;
        int accent = 0xFFFFAA22;
        context.fill(x - 6, y - 5, x + 112, y + 32, 0xB8101218);
        context.fill(x - 6, y - 5, x - 3, y + 32, accent);
        context.drawTextWithShadow(client.textRenderer, Text.literal(BRAND), x, y, accent);
        context.drawTextWithShadow(client.textRenderer, Text.literal("FPS  " + fps), x, y + 14, 0xFFFFFFFF);
    }

    public static void saveConfig() {
        CONFIG.save();
    }
}

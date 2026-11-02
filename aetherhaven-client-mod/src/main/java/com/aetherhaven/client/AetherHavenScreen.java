package com.aetherhaven.client;

import net.minecraft.client.gui.DrawContext;
import net.minecraft.client.gui.screen.Screen;
import net.minecraft.client.gui.widget.ButtonWidget;
import net.minecraft.text.Text;

public final class AetherHavenScreen extends Screen {
    private static final int BG = 0xF00B0D12;
    private static final int PANEL = 0xE0141720;
    private static final int ACCENT = 0xFFFFAA22;
    private static final int MUTED = 0xFF9AA3B2;

    public AetherHavenScreen() {
        super(Text.literal("AetherHaven Client"));
    }

    @Override
    protected void init() {
        int left = this.width / 2 - 155;
        int top = this.height / 2 - 108;

        addDrawableChild(ButtonWidget.builder(Text.literal(toggle("FPS HUD", AetherHavenClient.CONFIG.fpsHud)), button -> {
            AetherHavenClient.CONFIG.fpsHud = !AetherHavenClient.CONFIG.fpsHud;
            button.setMessage(Text.literal(toggle("FPS HUD", AetherHavenClient.CONFIG.fpsHud)));
            AetherHavenClient.saveConfig();
        }).dimensions(left + 18, top + 70, 136, 22).build());

        addDrawableChild(ButtonWidget.builder(Text.literal(toggle("Performance Mode", AetherHavenClient.CONFIG.performanceMode)), button -> {
            AetherHavenClient.CONFIG.performanceMode = !AetherHavenClient.CONFIG.performanceMode;
            button.setMessage(Text.literal(toggle("Performance Mode", AetherHavenClient.CONFIG.performanceMode)));
            AetherHavenClient.saveConfig();
        }).dimensions(left + 162, top + 70, 136, 22).build());

        addDrawableChild(ButtonWidget.builder(Text.literal(toggle("Menu Blur", AetherHavenClient.CONFIG.menuBlur)), button -> {
            AetherHavenClient.CONFIG.menuBlur = !AetherHavenClient.CONFIG.menuBlur;
            button.setMessage(Text.literal(toggle("Menu Blur", AetherHavenClient.CONFIG.menuBlur)));
            AetherHavenClient.saveConfig();
        }).dimensions(left + 18, top + 98, 136, 22).build());

        addDrawableChild(ButtonWidget.builder(Text.literal(toggle("Keystrokes HUD", AetherHavenClient.CONFIG.keystrokesHud)), button -> {
            AetherHavenClient.CONFIG.keystrokesHud = !AetherHavenClient.CONFIG.keystrokesHud;
            button.setMessage(Text.literal(toggle("Keystrokes HUD", AetherHavenClient.CONFIG.keystrokesHud)));
            AetherHavenClient.saveConfig();
        }).dimensions(left + 162, top + 98, 136, 22).build());

        addDrawableChild(ButtonWidget.builder(Text.literal("Done"), button -> close())
                .dimensions(left + 18, top + 150, 280, 24).build());
    }

    private static String toggle(String label, boolean enabled) {
        return label + "  " + (enabled ? "ON" : "OFF");
    }

    @Override
    public void render(DrawContext context, int mouseX, int mouseY, float delta) {
        renderBackground(context, mouseX, mouseY, delta);
        int left = this.width / 2 - 155;
        int top = this.height / 2 - 108;
        context.fill(0, 0, width, height, BG);
        context.fill(left, top, left + 316, top + 190, PANEL);
        context.fill(left, top, left + 316, top + 3, ACCENT);
        context.drawTextWithShadow(textRenderer, Text.literal("AetherHaven Client"), left + 18, top + 18, ACCENT);
        context.drawTextWithShadow(textRenderer, Text.literal("RIGHT SHIFT  •  Client Menu"), left + 18, top + 37, MUTED);
        context.drawTextWithShadow(textRenderer, Text.literal("Client Mods"), left + 18, top + 56, 0xFFFFFFFF);
        super.render(context, mouseX, mouseY, delta);
    }
}

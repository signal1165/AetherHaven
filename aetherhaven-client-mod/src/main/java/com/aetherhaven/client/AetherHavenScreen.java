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
        int top = this.height / 2 - 142;
        addToggle(left + 18, top + 68, "FPS HUD", () -> AetherHavenClient.CONFIG.fpsHud);
        addToggle(left + 162, top + 68, "Coordinates", () -> AetherHavenClient.CONFIG.coordinatesHud);
        addToggle(left + 18, top + 96, "Keystrokes", () -> AetherHavenClient.CONFIG.keystrokesHud);
        addToggle(left + 162, top + 96, "Sprint Toggle", () -> AetherHavenClient.CONFIG.sprintToggle);
        addToggle(left + 18, top + 124, "Privacy Mode", () -> AetherHavenClient.CONFIG.privacyMode);
        addToggle(left + 162, top + 124, "Particle Guard", () -> AetherHavenClient.CONFIG.particleLimiter);
        addToggle(left + 18, top + 152, "Performance", () -> AetherHavenClient.CONFIG.performanceMode);
        addToggle(left + 162, top + 152, "Menu Blur", () -> AetherHavenClient.CONFIG.menuBlur);
        addDrawableChild(ButtonWidget.builder(Text.literal("Done"), button -> close())
                .dimensions(left + 18, top + 190, 280, 24).build());
    }

    private void addToggle(int x, int y, String label, ToggleValue value) {
        addDrawableChild(ButtonWidget.builder(Text.literal(toggle(label, value.get())), button -> {
            flip(label);
            button.setMessage(Text.literal(toggle(label, value.get())));
        }).dimensions(x, y, 136, 22).build());
    }

    private static void flip(String label) {
        switch (label) {
            case "FPS HUD" -> AetherHavenClient.CONFIG.fpsHud = !AetherHavenClient.CONFIG.fpsHud;
            case "Coordinates" -> AetherHavenClient.CONFIG.coordinatesHud = !AetherHavenClient.CONFIG.coordinatesHud;
            case "Keystrokes" -> AetherHavenClient.CONFIG.keystrokesHud = !AetherHavenClient.CONFIG.keystrokesHud;
            case "Sprint Toggle" -> AetherHavenClient.CONFIG.sprintToggle = !AetherHavenClient.CONFIG.sprintToggle;
            case "Privacy Mode" -> AetherHavenClient.CONFIG.privacyMode = !AetherHavenClient.CONFIG.privacyMode;
            case "Particle Guard" -> AetherHavenClient.CONFIG.particleLimiter = !AetherHavenClient.CONFIG.particleLimiter;
            case "Performance" -> AetherHavenClient.CONFIG.performanceMode = !AetherHavenClient.CONFIG.performanceMode;
            case "Menu Blur" -> AetherHavenClient.CONFIG.menuBlur = !AetherHavenClient.CONFIG.menuBlur;
            default -> { }
        }
        AetherHavenClient.saveConfig();
    }

    private static String toggle(String label, boolean enabled) {
        return label + "  " + (enabled ? "ON" : "OFF");
    }

    @Override
    public void render(DrawContext context, int mouseX, int mouseY, float delta) {
        renderBackground(context, mouseX, mouseY, delta);
        int left = this.width / 2 - 155;
        int top = this.height / 2 - 142;
        context.fill(0, 0, width, height, BG);
        if (AetherHavenClient.CONFIG.menuBlur) {
            context.fill(0, 0, width, height, 0x22000000);
        }
        context.fill(left, top, left + 316, top + 226, PANEL);
        context.fill(left, top, left + 316, top + 3, ACCENT);
        context.drawTextWithShadow(textRenderer, Text.literal("AetherHaven Client"), left + 18, top + 18, ACCENT);
        context.drawTextWithShadow(textRenderer, Text.literal("RIGHT SHIFT  •  Client Menu"), left + 18, top + 37, MUTED);
        context.drawTextWithShadow(textRenderer, Text.literal("Independent HUD and performance controls"), left + 18, top + 53, 0xFFFFFFFF);
        super.render(context, mouseX, mouseY, delta);
    }

    @FunctionalInterface
    private interface ToggleValue { boolean get(); }
}

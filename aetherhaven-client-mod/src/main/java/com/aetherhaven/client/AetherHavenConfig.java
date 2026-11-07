package com.aetherhaven.client;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import net.fabricmc.loader.api.FabricLoader;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public final class AetherHavenConfig {
    private static final Gson GSON = new GsonBuilder().setPrettyPrinting().create();
    private static final Path PATH = FabricLoader.getInstance().getConfigDir().resolve("aetherhaven.json");

    public boolean fpsHud = true;
    public boolean coordinatesHud = false;
    public boolean keystrokesHud = false;
    public boolean performanceMode = true;
    public boolean menuBlur = true;
    public boolean sprintToggle = false;
    public boolean privacyMode = false;
    public boolean particleLimiter = true;
    public boolean weatherControl = false;
    public boolean timeControl = false;
    public int particleBudget = 160;

    public static AetherHavenConfig load() {
        try {
            if (Files.exists(PATH)) {
                AetherHavenConfig config = GSON.fromJson(Files.readString(PATH), AetherHavenConfig.class);
                return config == null ? new AetherHavenConfig() : config;
            }
        } catch (IOException | RuntimeException ignored) {
            // A broken config must never prevent Minecraft from starting.
        }
        return new AetherHavenConfig();
    }

    public void save() {
        try {
            Files.createDirectories(PATH.getParent());
            Files.writeString(PATH, GSON.toJson(this));
        } catch (IOException ignored) {
            // Configuration is best-effort and must not affect gameplay.
        }
    }
}

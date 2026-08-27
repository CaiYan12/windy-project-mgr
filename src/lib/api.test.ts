import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import {
  detectEditors,
  getAppInfo,
  getSettings,
  getWindowsAccentColor,
  updateSettings,
  type AppSettings,
} from "./api";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("settings v2 IPC wrappers", () => {
  beforeEach(() => {
    vi.mocked(invoke).mockReset();
  });

  it("uses the exact get_settings command and v2 return type", async () => {
    const settings: AppSettings = {
      colorMode: "system",
      accentColor: { kind: "preset", value: "windy-teal" },
      editor: { executable: "", arguments: ["{path}"] },
    };
    vi.mocked(invoke).mockResolvedValue(settings);

    await expect(getSettings()).resolves.toEqual(settings);
    expect(invoke).toHaveBeenCalledWith("get_settings");
  });

  it("uses the exact update_settings payload wrapper", async () => {
    const settings: AppSettings = {
      colorMode: "dark",
      accentColor: { kind: "custom", value: "#123456" },
      editor: { executable: "code", arguments: ["--reuse-window", "{path}"] },
    };
    vi.mocked(invoke).mockResolvedValue(settings);

    await expect(updateSettings(settings)).resolves.toEqual(settings);
    expect(invoke).toHaveBeenCalledWith("update_settings", { settings });
  });

  it("uses the exact system command names and Rust wire fields", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([])
      .mockResolvedValueOnce("#2A4C53")
      .mockResolvedValueOnce({ version: "0.1.0", dataDir: "D:/Windy/data" });

    await expect(detectEditors()).resolves.toEqual([]);
    await expect(getWindowsAccentColor()).resolves.toBe("#2A4C53");
    await expect(getAppInfo()).resolves.toEqual({ version: "0.1.0", dataDir: "D:/Windy/data" });
    expect(invoke).toHaveBeenNthCalledWith(1, "detect_editors");
    expect(invoke).toHaveBeenNthCalledWith(2, "get_windows_accent_color");
    expect(invoke).toHaveBeenNthCalledWith(3, "get_app_info");
  });
});

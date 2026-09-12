// 前端设置默认值的唯一来源（A4）：App 与 SettingsDialog 共同引用，
// 避免同一份 DEFAULT_SETTINGS 在多处硬编码后漂移。

import type { AppSettings } from "./api";
import { DEFAULT_APPEARANCE } from "./appearance";
import { DEFAULT_ACCENT_COLOR } from "./theme";

export const DEFAULT_SETTINGS: AppSettings = {
  colorMode: "system",
  accentColor: DEFAULT_ACCENT_COLOR,
  appearance: DEFAULT_APPEARANCE,
  editor: { executable: "", arguments: ["{path}"] },
};

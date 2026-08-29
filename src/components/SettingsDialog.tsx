import { useEffect, useRef, useState } from "react";
import type { KeyboardEvent, CSSProperties } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { Modal } from "./Modal";
import { Icon, type IconName } from "./Icon";
import { SettingsAppearancePanel } from "./SettingsAppearancePanel";
import { SettingsEditorPanel } from "./SettingsEditorPanel";
import { SettingsGeneralPanel } from "./SettingsGeneralPanel";
import {
  detectEditors,
  getAppInfo,
  getWindowsAccentColor,
  updateSettings,
  type AccentColor,
  type AppInfo,
  type AppSettings,
  type ColorMode,
  type DetectedEditor,
  type EditorProfile,
} from "../lib/api";
import {
  DEFAULT_ACCENT_COLOR,
  DEFAULT_EDITOR_ARGUMENTS,
  isValidHexColor,
  isValidTheme,
  validateEditorProfile,
} from "../lib/theme";
import {
  createRequestGeneration,
  isSettingsCloseBlocked,
} from "../lib/settingsUi";

type SettingsTab = "appearance" | "editor" | "general";

const TABS: ReadonlyArray<{ id: SettingsTab; label: string; icon: IconName }> = [
  { id: "appearance", label: "Appearance", icon: "palette" },
  { id: "editor", label: "Editor", icon: "code" },
  { id: "general", label: "General", icon: "info" },
];

const DEFAULT_SETTINGS: AppSettings = {
  colorMode: "system",
  accentColor: DEFAULT_ACCENT_COLOR,
  editor: {
    executable: "",
    arguments: [...DEFAULT_EDITOR_ARGUMENTS],
  },
};

interface SettingsDialogProps {
  settings: AppSettings | null;
  windowsAccentColor: string | null;
  settingsError: string | null;
  windowsAccentError: string | null;
  onClose: () => void;
  onWindowsAccentResult: (color: string | null, error: string | null) => void;
  onSaved: (settings: AppSettings) => void;
  onThemePreview: (
    colorMode: ColorMode,
    accentColor: AccentColor,
    windowsAccentColor: string | null,
  ) => void;
  onThemeRestore: () => void;
  onRetrySettings: () => void;
}

export function SettingsDialog({
  settings,
  windowsAccentColor: initialWindowsAccentColor,
  settingsError,
  windowsAccentError: initialWindowsAccentError,
  onClose,
  onWindowsAccentResult,
  onSaved,
  onThemePreview,
  onThemeRestore,
  onRetrySettings,
}: SettingsDialogProps) {
  const [activeTab, setActiveTab] = useState<SettingsTab>("appearance");
  const [colorMode, setColorMode] = useState<ColorMode>("system");
  const [accentColor, setAccentColor] = useState<AccentColor>(DEFAULT_ACCENT_COLOR);
  const [customAccentValue, setCustomAccentValue] = useState("#0E7D8C");
  const [editorProfile, setEditorProfile] = useState<EditorProfile>({
    executable: "",
    arguments: [...DEFAULT_EDITOR_ARGUMENTS],
  });
  const [argumentsText, setArgumentsText] = useState(DEFAULT_EDITOR_ARGUMENTS.join("\n"));
  const [windowsAccentColor, setWindowsAccentColor] = useState<string | null>(
    initialWindowsAccentColor,
  );
  const [windowsAccentError, setWindowsAccentError] = useState<string | null>(
    initialWindowsAccentError,
  );
  const [windowsAccentLoading, setWindowsAccentLoading] = useState(false);
  const [detectedEditors, setDetectedEditors] = useState<DetectedEditor[]>([]);
  const [detectingEditors, setDetectingEditors] = useState(false);
  const [editorDetectionError, setEditorDetectionError] = useState<string | null>(null);
  const [editorActionError, setEditorActionError] = useState<string | null>(null);
  const [appInfo, setAppInfo] = useState<AppInfo | null>(null);
  const [appInfoLoading, setAppInfoLoading] = useState(false);
  const [appInfoError, setAppInfoError] = useState<string | null>(null);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [resetConfirmOpen, setResetConfirmOpen] = useState(false);
  const [resetBusy, setResetBusy] = useState(false);
  const [generalFeedback, setGeneralFeedback] = useState<
    { kind: "success" | "error"; message: string } | null
  >(null);
  const tabRefs = useRef<Array<HTMLButtonElement | null>>([]);
  const editorsLoadedRef = useRef(false);
  const draftRef = useRef({ colorMode, accentColor });
  const draftDirtyRef = useRef(false);
  const windowsAccentRequestGeneration = useRef(createRequestGeneration());
  const editorRequestGeneration = useRef(createRequestGeneration());
  const appInfoRequestGeneration = useRef(createRequestGeneration());
  const resetConfirmTriggerRef = useRef<HTMLElement | null>(null);
  const resetConfirmFirstActionRef = useRef<HTMLButtonElement | null>(null);

  useEffect(() => {
    return () => {
      windowsAccentRequestGeneration.current.invalidate();
      editorRequestGeneration.current.invalidate();
      appInfoRequestGeneration.current.invalidate();
    };
  }, []);

  useEffect(() => {
    draftRef.current = { colorMode, accentColor };
  }, [colorMode, accentColor]);

  useEffect(() => {
    if (!settings) {
      return;
    }
    if (draftDirtyRef.current) {
      return;
    }
    setColorMode(isValidTheme(settings.colorMode) ? settings.colorMode : "system");
    setAccentColor(settings.accentColor);
    setCustomAccentValue(
      settings.accentColor.kind === "custom" ? settings.accentColor.value : "#0E7D8C",
    );
    setEditorProfile({
      executable: settings.editor.executable,
      arguments: [...settings.editor.arguments],
    });
    setArgumentsText(settings.editor.arguments.join("\n"));
  }, [settings]);

  async function readWindowsAccent() {
    const request = windowsAccentRequestGeneration.current.next();
    setWindowsAccentLoading(true);
    try {
      const value = await getWindowsAccentColor();
      if (!windowsAccentRequestGeneration.current.isCurrent(request)) {
        return;
      }
      if (!isValidHexColor(value)) {
        throw new Error("Windows accent color returned an invalid #RRGGBB value");
      }
      setWindowsAccentColor(value);
      setWindowsAccentError(null);
      onWindowsAccentResult(value, null);
      if (draftRef.current.accentColor.kind === "windows") {
        onThemePreview(draftRef.current.colorMode, draftRef.current.accentColor, value);
      }
    } catch (error) {
      if (!windowsAccentRequestGeneration.current.isCurrent(request)) {
        return;
      }
      setWindowsAccentColor(null);
      setWindowsAccentError(String(error));
      onWindowsAccentResult(null, String(error));
      if (draftRef.current.accentColor.kind === "windows") {
        onThemePreview(draftRef.current.colorMode, draftRef.current.accentColor, null);
      }
    } finally {
      if (windowsAccentRequestGeneration.current.isCurrent(request)) {
        setWindowsAccentLoading(false);
      }
    }
  }

  useEffect(() => {
    void readWindowsAccent();
    // The accent is read once when this dialog opens. Draft changes use the local value.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function loadEditors() {
    const request = editorRequestGeneration.current.next();
    setDetectingEditors(true);
    setEditorDetectionError(null);
    try {
      const editors = await detectEditors();
      if (editorRequestGeneration.current.isCurrent(request)) {
        setDetectedEditors(editors);
      }
    } catch (error) {
      if (editorRequestGeneration.current.isCurrent(request)) {
        setDetectedEditors([]);
        setEditorDetectionError(String(error));
      }
    } finally {
      if (editorRequestGeneration.current.isCurrent(request)) {
        setDetectingEditors(false);
      }
    }
  }

  async function loadAppInfo() {
    const request = appInfoRequestGeneration.current.next();
    setAppInfoLoading(true);
    setAppInfoError(null);
    try {
      const info = await getAppInfo();
      if (appInfoRequestGeneration.current.isCurrent(request)) {
        setAppInfo(info);
      }
    } catch (error) {
      if (appInfoRequestGeneration.current.isCurrent(request)) {
        setAppInfo(null);
        setAppInfoError(String(error));
      }
    } finally {
      if (appInfoRequestGeneration.current.isCurrent(request)) {
        setAppInfoLoading(false);
      }
    }
  }

  useEffect(() => {
    if (activeTab === "editor" && !editorsLoadedRef.current) {
      editorsLoadedRef.current = true;
      void loadEditors();
    }
    if (activeTab === "general") {
      void loadAppInfo();
    }
  }, [activeTab]);

  useEffect(() => {
    if (!resetConfirmOpen) {
      return;
    }
    resetConfirmFirstActionRef.current?.focus();
    return () => {
      resetConfirmTriggerRef.current?.focus();
      resetConfirmTriggerRef.current = null;
    };
  }, [resetConfirmOpen]);

  function changeTab(tab: SettingsTab) {
    setActiveTab(tab);
    const index = TABS.findIndex((item) => item.id === tab);
    tabRefs.current[index]?.focus();
  }

  function onTabKeyDown(event: KeyboardEvent<HTMLButtonElement>, index: number) {
    let nextIndex: number | null = null;
    if (event.key === "ArrowRight") {
      nextIndex = (index + 1) % TABS.length;
    } else if (event.key === "ArrowLeft") {
      nextIndex = (index - 1 + TABS.length) % TABS.length;
    } else if (event.key === "Home") {
      nextIndex = 0;
    } else if (event.key === "End") {
      nextIndex = TABS.length - 1;
    }
    if (nextIndex !== null) {
      event.preventDefault();
      changeTab(TABS[nextIndex].id);
    }
  }

  function preview(
    colorModeValue: ColorMode,
    accentColorValue: AccentColor,
    windowsColor = windowsAccentColor,
  ) {
    draftDirtyRef.current = true;
    draftRef.current = { colorMode: colorModeValue, accentColor: accentColorValue };
    setColorMode(colorModeValue);
    setAccentColor(accentColorValue);
    onThemePreview(colorModeValue, accentColorValue, windowsColor);
  }

  function changeAccent(nextAccentColor: AccentColor) {
    if (nextAccentColor.kind === "custom") {
      setCustomAccentValue(nextAccentColor.value);
    }
    preview(colorMode, nextAccentColor);
  }

  function changeCustomAccent(value: string) {
    setCustomAccentValue(value);
    preview(colorMode, { kind: "custom", value });
  }

  function changeArguments(value: string) {
    draftDirtyRef.current = true;
    setArgumentsText(value);
    setEditorProfile((profile) => ({
      ...profile,
      arguments: value.split(/\r?\n/),
    }));
  }

  function selectExecutable(executable: string) {
    draftDirtyRef.current = true;
    setEditorActionError(null);
    setEditorProfile((profile) => ({ ...profile, executable }));
  }

  async function chooseOtherExecutable() {
    setEditorActionError(null);
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        filters: [{ name: "Executable", extensions: ["exe"] }],
      });
      if (!selected) {
        return;
      }
      if (typeof selected !== "string" || !selected.toLowerCase().endsWith(".exe")) {
        setEditorActionError("Choose an executable file with the .exe extension.");
        return;
      }
      selectExecutable(selected);
    } catch (error) {
      setEditorActionError(String(error));
    }
  }

  const editorValidationError = validateEditorProfile(editorProfile);
  const accentValidationError =
    accentColor.kind === "custom" && !isValidHexColor(customAccentValue)
      ? "Use a six-digit color in the format #RRGGBB."
      : null;
  const isSettingsBusy = isSettingsCloseBlocked(busy, resetBusy);

  async function save() {
    setSaveError(null);
    setBusy(true);
    try {
      const saved = await updateSettings({
        colorMode,
        accentColor,
        editor: {
          executable: editorProfile.executable.trim(),
          arguments: [...editorProfile.arguments],
        },
      });
       onSaved(saved);
       draftDirtyRef.current = false;
       windowsAccentRequestGeneration.current.invalidate();
       editorRequestGeneration.current.invalidate();
       appInfoRequestGeneration.current.invalidate();
       onClose();
    } catch (error) {
      setSaveError(String(error));
    } finally {
      setBusy(false);
    }
  }

  async function resetSettings() {
    setGeneralFeedback(null);
    setResetBusy(true);
    try {
      const saved = await updateSettings({
        colorMode: DEFAULT_SETTINGS.colorMode,
        accentColor: { ...DEFAULT_SETTINGS.accentColor },
        editor: {
          executable: DEFAULT_SETTINGS.editor.executable,
          arguments: [...DEFAULT_SETTINGS.editor.arguments],
        },
      });
      setColorMode(saved.colorMode);
      setAccentColor(saved.accentColor);
      setCustomAccentValue("#0E7D8C");
       setEditorProfile({
         executable: saved.editor.executable,
         arguments: [...saved.editor.arguments],
       });
       setArgumentsText(saved.editor.arguments.join("\n"));
       draftDirtyRef.current = false;
       onThemePreview(saved.colorMode, saved.accentColor, windowsAccentColor);
      onSaved(saved);
      setResetConfirmOpen(false);
      setGeneralFeedback({ kind: "success", message: "Settings reset to the Rust defaults." });
    } catch (error) {
      setGeneralFeedback({ kind: "error", message: `Could not reset settings: ${String(error)}` });
    } finally {
      setResetBusy(false);
    }
  }

  function cancel() {
    if (isSettingsBusy) {
      return;
    }
    windowsAccentRequestGeneration.current.invalidate();
    editorRequestGeneration.current.invalidate();
    appInfoRequestGeneration.current.invalidate();
    onThemeRestore();
    onClose();
  }

  function openResetConfirmation() {
    resetConfirmTriggerRef.current = document.activeElement instanceof HTMLElement
      ? document.activeElement
      : null;
    setResetConfirmOpen(true);
  }

  if (!settings) {
    return (
      <Modal
        title="Settings"
        onClose={cancel}
        closeDisabled={isSettingsBusy}
        wide
        footer={
          <button className="btn" onClick={cancel} disabled={isSettingsBusy}>
            <Icon name="close" size={15} />
            <span>Close</span>
          </button>
        }
      >
        <div className="settings-loading-state">
          <p className="muted detail-inline-message">
            <Icon name="settings" size={18} />
            <span>Loading settings…</span>
          </p>
          {settingsError && (
            <div className="settings-inline-alert" role="alert">
              <div className="inline-alert-copy">
                <Icon name="alert-triangle" size={17} />
                <span>{settingsError}</span>
              </div>
              <button type="button" className="link-btn" onClick={onRetrySettings}>
                <Icon name="refresh" size={14} />
                Try again
              </button>
            </div>
          )}
        </div>
      </Modal>
    );
  }

  return (
    <Modal
      title="Settings"
      onClose={cancel}
      closeDisabled={isSettingsBusy}
      wide
      footer={
        <>
          <button type="button" className="btn" onClick={cancel} disabled={isSettingsBusy}>
            <Icon name="close" size={15} />
            <span>Cancel</span>
          </button>
          <button
            type="button"
            className="btn btn-primary"
            onClick={save}
            disabled={
              busy || resetBusy || windowsAccentLoading || editorValidationError !== null || accentValidationError !== null
            }
          >
            <Icon name="check-circle" size={15} />
            <span>{busy ? "Saving…" : "Save changes"}</span>
          </button>
        </>
      }
    >
      <div className="settings-dialog-content">
        {settingsError && (
          <div className="settings-inline-alert" role="alert">
            <div className="inline-alert-copy">
              <Icon name="alert-triangle" size={17} />
              <span>{settingsError} The v2 defaults are in use until settings reloads.</span>
            </div>
            <button type="button" className="link-btn" onClick={onRetrySettings}>
              <Icon name="refresh" size={14} />
              Try again
            </button>
          </div>
        )}

        <div className="settings-tabs-shell">
          <div className="settings-tabs" role="tablist" aria-label="Settings sections">
            {TABS.map((tab, index) => {
              const selected = activeTab === tab.id;
              return (
                <button
                  key={tab.id}
                  ref={(element) => {
                    tabRefs.current[index] = element;
                  }}
                  type="button"
                  className="settings-tab"
                  id={`settings-tab-${tab.id}`}
                  role="tab"
                  aria-selected={selected}
                  aria-controls={`settings-panel-${tab.id}`}
                  tabIndex={selected ? 0 : -1}
                  onClick={() => changeTab(tab.id)}
                  onKeyDown={(event) => onTabKeyDown(event, index)}
                >
                  <Icon name={tab.icon} size={16} />
                  {tab.label}
                </button>
              );
            })}
            <span
              className="settings-tab-indicator"
              aria-hidden="true"
              style={{ "--settings-tab-index": TABS.findIndex((tab) => tab.id === activeTab) } as CSSProperties}
            />
          </div>
        </div>

        {TABS.map((tab) => (
          <div
            key={tab.id}
            id={`settings-panel-${tab.id}`}
            className="settings-tabpanel"
            role="tabpanel"
            aria-labelledby={`settings-tab-${tab.id}`}
            tabIndex={0}
            hidden={activeTab !== tab.id}
          >
            {tab.id === "appearance" && (
              <SettingsAppearancePanel
                colorMode={colorMode}
                accentColor={accentColor}
                customAccentValue={customAccentValue}
                windowsAccentColor={windowsAccentColor}
                windowsAccentLoading={windowsAccentLoading}
                windowsAccentError={windowsAccentError}
                onColorModeChange={(next) => preview(next, accentColor)}
                onAccentChange={changeAccent}
                onCustomAccentChange={changeCustomAccent}
                onRetryWindowsAccent={() => void readWindowsAccent()}
              />
            )}
            {tab.id === "editor" && (
              <SettingsEditorPanel
                profile={editorProfile}
                argumentsText={argumentsText}
                detectedEditors={detectedEditors}
                loading={detectingEditors}
                error={editorDetectionError ?? editorActionError}
                validationError={editorValidationError}
                onSelectExecutable={selectExecutable}
                onArgumentsChange={changeArguments}
                onChooseOther={() => void chooseOtherExecutable()}
                onRetry={() => void loadEditors()}
              />
            )}
            {tab.id === "general" && (
              <SettingsGeneralPanel
                appInfo={appInfo}
                loading={appInfoLoading}
                error={appInfoError}
                feedback={generalFeedback}
                 resetBusy={resetBusy}
                 onRetry={() => void loadAppInfo()}
                 onReset={openResetConfirmation}
               />
            )}
          </div>
        ))}

        {resetConfirmOpen && (
          <div
             className="settings-reset-confirm"
             role="alertdialog"
             aria-labelledby="settings-reset-confirm-title"
             aria-describedby="settings-reset-confirm-description"
           >
             <div className="settings-reset-confirm-copy">
               <h3 id="settings-reset-confirm-title">
                 <Icon name="rotate-ccw" size={18} />
                 <span>Reset all settings?</span>
               </h3>
               <p id="settings-reset-confirm-description">
                 This resets color, accent, and editor settings. Project records stay untouched.
               </p>
             </div>
             <div className="settings-reset-actions">
               <button
                 type="button"
                 className="btn small"
                 ref={resetConfirmFirstActionRef}
                 onClick={() => setResetConfirmOpen(false)}
                disabled={resetBusy}
              >
                <Icon name="close" size={14} />
                <span>
                Keep settings
                </span>
              </button>
              <button
                type="button"
                className="btn btn-danger small"
                onClick={() => void resetSettings()}
                disabled={resetBusy}
              >
                <Icon name="rotate-ccw" size={14} />
                <span>
                Reset now
                </span>
              </button>
            </div>
          </div>
        )}

        {saveError && (
          <p className="settings-error settings-inline-message" role="alert">
            <Icon name="alert-triangle" size={16} />
            <span>{saveError}</span>
          </p>
        )}
      </div>
    </Modal>
  );
}

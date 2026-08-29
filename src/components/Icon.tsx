import { useId, type ReactNode, type SVGProps } from "react";

export type IconName =
  | "windy"
  | "layout-grid"
  | "tag"
  | "settings"
  | "search"
  | "plus"
  | "folder-open"
  | "play"
  | "hammer"
  | "more-horizontal"
  | "code"
  | "pencil"
  | "trash"
  | "refresh"
  | "arrow-left"
  | "close"
  | "file-script"
  | "ban"
  | "git-branch"
  | "check-circle"
  | "alert-triangle"
  | "help-circle"
  | "activity"
  | "overview"
  | "layers"
  | "history"
  | "zap"
  | "palette"
  | "sun"
  | "moon"
  | "monitor"
  | "info"
  | "rotate-ccw"
  | "chevron-left"
  | "chevron-right"
  | "text"
  | "align-left"
  | "terminal"
  | "folder-plus";

export const ICON_NAMES: ReadonlyArray<IconName> = [
  "windy",
  "layout-grid",
  "tag",
  "settings",
  "search",
  "plus",
  "folder-open",
  "play",
  "hammer",
  "more-horizontal",
  "code",
  "pencil",
  "trash",
  "refresh",
  "arrow-left",
  "close",
  "file-script",
  "ban",
  "git-branch",
  "check-circle",
  "alert-triangle",
  "help-circle",
  "activity",
  "overview",
  "layers",
  "history",
  "zap",
  "palette",
  "sun",
  "moon",
  "monitor",
  "info",
  "rotate-ccw",
  "chevron-left",
  "chevron-right",
  "text",
  "align-left",
  "terminal",
  "folder-plus",
];

export interface IconProps extends Omit<SVGProps<SVGSVGElement>, "children"> {
  name: IconName;
  size?: number;
  strokeWidth?: number;
  title?: string;
}

export function Icon({
  name,
  size = 18,
  strokeWidth = 1.75,
  title,
  className,
  ...props
}: IconProps) {
  const titleId = useId();

  return (
    <svg
      {...props}
      className={`icon${className ? ` ${className}` : ""}`}
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={strokeWidth}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden={title ? undefined : true}
      aria-labelledby={title ? titleId : undefined}
      role={title ? "img" : undefined}
      focusable="false"
    >
      {title && <title id={titleId}>{title}</title>}
      {renderIcon(name)}
    </svg>
  );
}

function renderIcon(name: IconName): ReactNode {
  switch (name) {
    case "windy":
      return (
        <>
          <path d="M3 8.25h12.25c1.65 0 2.75-.82 2.75-2.1 0-1.1-.86-1.9-1.95-1.9-.8 0-1.48.38-1.9 1" />
          <path d="M3 12h16.25c1.65 0 2.75.82 2.75 2.1 0 1.1-.86 1.9-1.95 1.9-.8 0-1.48-.38-1.9-1" />
          <path d="M3 15.75h9.25c1.65 0 2.75.82 2.75 2.1 0 1.1-.86 1.9-1.95 1.9-.8 0-1.48-.38-1.9-1" />
        </>
      );
    case "layout-grid":
      return (
        <>
          <rect x="3.5" y="3.5" width="7" height="7" rx="1" />
          <rect x="13.5" y="3.5" width="7" height="7" rx="1" />
          <rect x="3.5" y="13.5" width="7" height="7" rx="1" />
          <rect x="13.5" y="13.5" width="7" height="7" rx="1" />
        </>
      );
    case "tag":
      return <path d="M4 5.25v5.1l8.45 8.45a1.7 1.7 0 0 0 2.4 0l3.95-3.95a1.7 1.7 0 0 0 0-2.4L10.35 4H5.25A1.25 1.25 0 0 0 4 5.25Z M7.3 7.3h.01" />;
    case "settings":
      return (
        <>
          <path d="m9.9 4.35.55-1.1h3.1l.55 1.1 1.25.7 1.2-.25 2.2 2.2-.25 1.2.7 1.25 1.1.55v3.1l-1.1.55-.7 1.25.25 1.2-2.2 2.2-1.2-.25-1.25.7-.55 1.1h-3.1l-.55-1.1-1.25-.7-1.2.25-2.2-2.2.25-1.2-.7-1.25-1.1-.55V10.2l1.1-.55.7-1.25-.25-1.2 2.2-2.2 1.2.25 1.25-.7Z" />
          <circle cx="12" cy="12" r="3.1" />
        </>
      );
    case "search":
      return (
        <>
          <circle cx="10.75" cy="10.75" r="6.25" />
          <path d="m16 16 4.5 4.5" />
        </>
      );
    case "plus":
      return <><path d="M12 5v14" /><path d="M5 12h14" /></>;
    case "folder-open":
      return <path d="M3.5 7.75A1.75 1.75 0 0 1 5.25 6h4l2 2h7.5A1.75 1.75 0 0 1 20.5 9.75v.5l-1.55 6.2A2 2 0 0 1 17 18H5.25a1.75 1.75 0 0 1-1.7-2.15L5 10.5h15" />;
    case "play":
      return <path d="m8.25 5.75 10 6.25-10 6.25V5.75Z" />;
    case "hammer":
      return (
        <path
          d="M968.533333 810.666667l-388.266666-388.266667c38.4-98.133333 17.066667-213.333333-64-294.4-85.333333-85.333333-213.333333-102.4-315.733334-55.466667L384 256 256 384 68.266667 200.533333C17.066667 302.933333 38.4 430.933333 123.733333 516.266667c81.066667 81.066667 196.266667 102.4 294.4 64l388.266667 388.266666c17.066667 17.066667 42.666667 17.066667 59.733333 0l98.133334-98.133333c21.333333-17.066667 21.333333-46.933333 4.266666-59.733333z"
          fill="currentColor"
          stroke="none"
          transform="scale(0.0234375)"
        />
      );
    case "more-horizontal":
      return <><circle cx="5" cy="12" r="1" fill="currentColor" stroke="none" /><circle cx="12" cy="12" r="1" fill="currentColor" stroke="none" /><circle cx="19" cy="12" r="1" fill="currentColor" stroke="none" /></>;
    case "code":
      return <><path d="m8.25 7.5-4.5 4.5 4.5 4.5" /><path d="m15.75 7.5 4.5 4.5-4.5 4.5" /><path d="m13.75 4-3.5 16" /></>;
    case "pencil":
      return <><path d="m4.25 16.75-.75 3.75 3.75-.75L19 8l-3-3L4.25 16.75Z" /><path d="m13.75 6.25 3 3" /></>;
    case "trash":
      return <><path d="M5 7.5h14" /><path d="M9 7.5v-2h6v2" /><path d="m7 7.5.75 12h8.5L17 7.5" /><path d="M10 11v5.25 M14 11v5.25" /></>;
    case "refresh":
      return <><path d="M20 11a8 8 0 0 0-13.55-5.75L4 7.7" /><path d="M4 4.25v3.45h3.45" /><path d="M4 13a8 8 0 0 0 13.55 5.75L20 16.3" /><path d="M20 19.75V16.3h-3.45" /></>;
    case "arrow-left":
      return <><path d="M19.5 12H4.5" /><path d="m10 5.75-6.25 6.25L10 18.25" /></>;
    case "close":
      return <><path d="m6 6 12 12" /><path d="M18 6 6 18" /></>;
    case "file-script":
      return <><path d="M6 3.5h7l5 5v12H6a2 2 0 0 1-2-2v-13a2 2 0 0 1 2-2Z" /><path d="M13 3.5v5h5" /><path d="m8 13 2 2-2 2 M12 17h4" /></>;
    case "ban":
      return <><circle cx="12" cy="12" r="8.5" /><path d="m6 6 12 12" /></>;
    case "git-branch":
      return <><circle cx="6.25" cy="5.75" r="2.25" /><circle cx="17.75" cy="18.25" r="2.25" /><circle cx="17.75" cy="6.25" r="2.25" /><path d="M6.25 8v4a6.25 6.25 0 0 0 6.25 6.25h3" /><path d="M6.25 8v1.5A5.75 5.75 0 0 0 12 15.25h3.5" /></>;
    case "check-circle":
      return <><circle cx="12" cy="12" r="8.5" /><path d="m8 12.25 2.65 2.65L16.5 9" /></>;
    case "alert-triangle":
      return <><path d="m10.55 4.45-7 12.1A1.7 1.7 0 0 0 5 19.1h14a1.7 1.7 0 0 0 1.45-2.55l-7-12.1a1.7 1.7 0 0 0-2.9 0Z" /><path d="M12 9v4.25 M12 16.5h.01" /></>;
    case "help-circle":
      return <><circle cx="12" cy="12" r="8.5" /><path d="M9.75 9.25a2.35 2.35 0 0 1 4.5 1c0 1.7-2.25 1.9-2.25 3.45" /><path d="M12 16.75h.01" /></>;
    case "activity":
      return <path d="M3.5 12h3l2-5 3.2 10 2.3-6 1.5 3h5" />;
    case "overview":
      return <><rect x="4" y="4" width="16" height="16" rx="2" /><path d="M8 9h8 M8 12.5h8 M8 16h4" /></>;
    case "layers":
      return <><path d="m12 3.75 8 4.25-8 4.25-8-4.25 8-4.25Z" /><path d="m4 12 8 4.25L20 12" /><path d="m4 16.25 8 4.25 8-4.25" /></>;
    case "history":
      return <><path d="M4.25 11a7.75 7.75 0 1 1 2.25 5.48" /><path d="M4.25 5.5v5.5h5.5" /><path d="M12 7.75V12l3 1.75" /></>;
    case "zap":
      return <path d="M13.5 2.75 5.25 13h6l-.75 8.25L18.75 11h-6l.75-8.25Z" />;
    case "palette":
      return <><path d="M12 3.75a8.25 8.25 0 0 0 0 16.5h1.2a1.55 1.55 0 0 0 1.05-2.7 1.55 1.55 0 0 1 1.05-2.7H17A3.25 3.25 0 0 0 20.25 11.6 7.75 7.75 0 0 0 12 3.75Z" /><path d="M7.5 10h.01 M9.5 7.25h.01 M14 7.25h.01 M16.5 10h.01" /></>;
    case "sun":
      return <><circle cx="12" cy="12" r="3.5" /><path d="M12 2.75v2 M12 19.25v2 M21.25 12h-2 M4.75 12h-2 M18.54 5.46l-1.42 1.42 M6.88 17.12l-1.42 1.42 M18.54 18.54l-1.42-1.42 M6.88 6.88 5.46 5.46" /></>;
    case "moon":
      return <path d="M19.75 15.4A7.75 7.75 0 0 1 8.6 4.25 8.5 8.5 0 1 0 19.75 15.4Z" />;
    case "monitor":
      return <><rect x="3.5" y="4" width="17" height="12" rx="1.75" /><path d="M8.5 20h7 M12 16v4" /></>;
    case "info":
      return <><circle cx="12" cy="12" r="8.5" /><path d="M12 10.75v5 M12 7.75h.01" /></>;
    case "rotate-ccw":
      return <><path d="M4.25 10a8 8 0 1 1 2.35 5.65" /><path d="M4.25 4.5V10h5.5" /></>;
    case "chevron-left":
      return <path d="m14.5 5.75-6.25 6.25 6.25 6.25" />;
    case "chevron-right":
      return <path d="m9.5 5.75 6.25 6.25-6.25 6.25" />;
    case "text":
      return <><path d="M5 5h14 M12 5v14 M8.5 19h7" /></>;
    case "align-left":
      return <><path d="M4 6h16 M4 10.5h11 M4 15h16 M4 19.5h9" /></>;
    case "terminal":
      return <><path d="m5 7 4.5 5L5 17" /><path d="M12.5 17H19" /></>;
    case "folder-plus":
      return <><path d="M3.5 7.75A1.75 1.75 0 0 1 5.25 6h4l2 2h7.5a1.75 1.75 0 0 1 1.75 1.75v7.5A1.75 1.75 0 0 1 18.75 19H5.25a1.75 1.75 0 0 1-1.75-1.75V7.75Z" /><path d="M12 11v5 M9.5 13.5h5" /></>;
  }
}

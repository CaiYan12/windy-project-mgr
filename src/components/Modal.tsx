// 对话框基础件：Modal 壳（Esc / 遮罩关闭）与确认对话框（删除等破坏性操作）。

import { useLayoutEffect, useRef, type ReactNode } from "react";
import { Icon } from "./Icon";

export function Modal({
  title,
  onClose,
  children,
  footer,
  narrow,
  wide,
  closeDisabled = false,
}: {
  title: string;
  onClose: () => void;
  children: ReactNode;
  footer?: ReactNode;
  /** 窄变体：确认类对话框用（横线更短、与卡片比例接近）。 */
  narrow?: boolean;
  /** 宽变体：设置中心等多分区对话框使用。 */
  wide?: boolean;
  /** 阻止 Escape、遮罩和关闭按钮在未决操作期间脱离对话框。 */
  closeDisabled?: boolean;
}) {
  const dialogRef = useRef<HTMLDivElement>(null);
  const previousActiveElementRef = useRef<HTMLElement | null>(null);

  useLayoutEffect(() => {
    previousActiveElementRef.current = document.activeElement instanceof HTMLElement
      ? document.activeElement
      : null;
    const dialog = dialogRef.current;
    if (!dialog) {
      return;
    }
    const firstFocusable = getFocusableElements(dialog)[0];
    (firstFocusable ?? dialog).focus();

    return () => {
      previousActiveElementRef.current?.focus();
    };
  }, []);

  useLayoutEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        if (closeDisabled) {
          e.preventDefault();
        } else {
          onClose();
        }
        return;
      }
      if (e.key !== "Tab") {
        return;
      }

      const dialog = dialogRef.current;
      if (!dialog) {
        return;
      }
      const focusable = getFocusableElements(dialog);
      if (focusable.length === 0) {
        e.preventDefault();
        dialog.focus();
        return;
      }

      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (e.shiftKey && document.activeElement === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [closeDisabled, onClose]);

  return (
    <div
      className="modal-backdrop"
      onMouseDown={(e) => {
        if (!closeDisabled && e.target === e.currentTarget) {
          onClose();
        }
      }}
    >
      <div
        className={`modal${narrow ? " narrow" : ""}${wide ? " wide" : ""}`}
        ref={dialogRef}
        role="dialog"
        aria-modal="true"
        aria-label={title}
        tabIndex={-1}
      >
        <header className="modal-header">
          <h2>{title}</h2>
          <button
            className="icon-btn"
            onClick={() => {
              if (!closeDisabled) {
                onClose();
              }
            }}
            aria-label="Close"
            title="Close"
            disabled={closeDisabled}
          >
            <Icon name="close" size={18} />
          </button>
        </header>
        <div className="modal-body">{children}</div>
        {footer && <footer className="modal-footer">{footer}</footer>}
      </div>
    </div>
  );
}

function getFocusableElements(dialog: HTMLElement): HTMLElement[] {
  return Array.from(
    dialog.querySelectorAll<HTMLElement>(
      'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])',
    ),
  ).filter((element) => !element.closest("[hidden]"));
}

export function ConfirmDialog({
  title,
  message,
  confirmLabel,
  onConfirm,
  onCancel,
}: {
  title: string;
  message: ReactNode;
  confirmLabel: string;
  onConfirm: () => void;
  onCancel: () => void;
}) {
  return (
    <Modal
      title={title}
      onClose={onCancel}
      narrow
      footer={
        <>
          <button className="btn" onClick={onCancel}>
            <Icon name="close" size={15} />
            <span>Cancel</span>
          </button>
          <button className="btn btn-danger" onClick={onConfirm}>
            <Icon name="trash" size={15} />
            <span>{confirmLabel}</span>
          </button>
        </>
      }
    >
      <p className="confirm-message">{message}</p>
    </Modal>
  );
}

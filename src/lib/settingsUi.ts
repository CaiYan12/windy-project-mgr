export function isSettingsCloseBlocked(busy: boolean, resetBusy: boolean): boolean {
  return busy || resetBusy;
}

export function shouldShowCustomExecutable(
  executable: string,
  detectedExecutables: string[],
): boolean {
  return (
    executable.trim() !== "" &&
    !detectedExecutables.some(
      (detectedExecutable) =>
        detectedExecutable.toLowerCase() === executable.toLowerCase(),
    )
  );
}

export function nextRadioIndex(
  index: number,
  key: string,
  itemCount: number,
): number | null {
  if (itemCount <= 0) {
    return null;
  }
  if (key === "ArrowRight" || key === "ArrowDown") {
    return (index + 1) % itemCount;
  }
  if (key === "ArrowLeft" || key === "ArrowUp") {
    return (index - 1 + itemCount) % itemCount;
  }
  if (key === "Home") {
    return 0;
  }
  if (key === "End") {
    return itemCount - 1;
  }
  return null;
}

export function createRequestGeneration() {
  let current = 0;
  return {
    next() {
      current += 1;
      return current;
    },
    invalidate() {
      current += 1;
    },
    isCurrent(request: number) {
      return request === current;
    },
  };
}

import * as React from 'react';
import ResizeObserver, { ResizeObserverEntry } from 'resize-observer-polyfill';

export function useMeasure<T extends Element, A, B>(
  element: T | undefined | null,
  defaultWidth: A,
  defaultHeight: B,
): { width: number | A, height: number | B } {
  const [width, setWidth] = React.useState<number | A>(defaultWidth);
  const [height, setHeight] = React.useState<number | B>(defaultHeight);

  React.useEffect(() => {
    if (element == null) return;
    const observer = new ResizeObserver((entires) => {
      const entry = entires[0] as ResizeObserverEntry;
      const { width, height } = entry.contentRect;
      setHeight(height);
      setWidth(width);
    });

    observer.observe(element);

    return () => {
      observer.disconnect();
    };
  }, [element]);

  return { width, height };
}

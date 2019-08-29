// probably didn't need to go here but eh
declare module "resize-observer-polyfill" {
  export interface ResizeObserverEntry {
    /**
     * The Element whose size has changed.
     */
    readonly target: Element;

    /**
     * Element's content rect when ResizeObserverCallback is invoked.
     */
    readonly contentRect: DOMRectReadOnly;
  }

  export interface ResizeObserverCallback {
    (entries: ResizeObserverEntry[], observer: ResizeObserver): void;
  }

  export default class ResizeObserver {
    constructor(fn: ResizeObserverCallback);
    disconnect(): void;
    observe(element: Element): void;
  }
}

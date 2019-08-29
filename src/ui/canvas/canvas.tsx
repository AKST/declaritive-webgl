import * as React from 'react';
import { ViewportDimensions } from 'ui/viewport/viewport_dimensions';
import styles from './canvas.css';

export type CanvasProps<T> = {
  useRuntime(
    canvas: HTMLCanvasElement | undefined,
    runtimeModule: T | undefined,
    width: number,
    height: number,
  ): void;
  runtimePromise: Promise<T>,
};

export const Canvas = React.memo(function Canvas<T>(props: CanvasProps<T>) {
  const { runtimePromise, useRuntime } = props;
  const [runtime, setRuntime] = React.useState<T | undefined>();
  const [canvasEl, setCanvasEl] = React.useState<HTMLCanvasElement | null>(null);
  const { width, height } = React.useContext(ViewportDimensions);

  React.useEffect(function recordRuntime() {
    let active = true;

    runtimePromise.then(runtime => {
      if (active) setRuntime(runtime);

      return () => {
        active = false;
        setRuntime(undefined);
      };
    });
  }, [runtimePromise]);

  useRuntime(
    canvasEl || undefined,
    runtime,
    width,
    height,
  );

  return (
      <div className={styles.background}>
        <canvas
            ref={setCanvasEl}
            className={styles.canvas}
            width={width}
            height={height}
        />
      </div>
  );
});

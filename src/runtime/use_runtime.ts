import * as React from 'react';
import { checkExists } from 'base/types/types';
import { useSingletonFactory } from 'hooks/use_singleton_factory';
import { RuntimeModule, Runtime } from 'runtime/types';

export function useRuntime(
    canvas: HTMLCanvasElement | undefined,
    module: RuntimeModule | undefined,
    width: number,
    height: number,
    onError: (e: Error) => void,
    vertShader: string,
    fragShader: string,
) {
  const [runtime, setRuntime] = React.useState<Runtime | undefined>();
  const getContext = createContextFactory(canvas);

  React.useEffect(function setupPanicHook() {
    if (module == null) return;
    module.setupPanicHook();
  }, [module]);


  React.useEffect(function initializeRuntime() {
    if (module == null || canvas == null) return;

    const context = getContext(canvas);
    const builder = new module.RuntimeBuilder();

    try {
      builder.linkWebglContext(context);
      builder.linkVertShader(vertShader);
      builder.linkFragShader(fragShader);
      builder.setDimensions(width, height);

      const runtime = builder.createRuntime();
      runtime.tick();
      setRuntime(runtime);

      builder.free();

      return () => {
        setRuntime(undefined);
        runtime.free();
      };
    } catch (e) {
      onError(e);
    }
  }, [fragShader, vertShader, canvas, module]);

  React.useEffect(function onResize() {
    if (runtime == null) return;

    try {
      runtime.setDimensions(width, height);
    } catch (e) {
      onError(e);
    }
  }, [width, height, runtime]);

  React.useEffect(function animationLoop() {
    if (runtime == null) return;

    let animationFrame = requestAnimationFrame(function frame() {
      runtime.tick();
      requestAnimationFrame(frame);
    });

    return () => cancelAnimationFrame(animationFrame);
  }, [runtime]);
}

function createContextFactory(canvas?: HTMLCanvasElement) {
  return useSingletonFactory((canvas: HTMLCanvasElement): WebGLRenderingContext => {
    // TODO(Angus): fix casting
    return checkExists(
        canvas.getContext("webgl2"),
        'webgl is needed to run this',
    ) as WebGLRenderingContext;
  }, [canvas]);
}

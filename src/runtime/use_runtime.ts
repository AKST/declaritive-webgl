import * as React from 'react';
import { checkExists } from 'base/types/types';
import { useSingletonFactory } from 'hooks/use_singleton_factory';
import { RuntimeModule } from 'runtime/types';

export function useRuntime(
    canvas: HTMLCanvasElement | undefined,
    module: RuntimeModule | undefined,
    width: number,
    height: number,
    onError: (e: Error) => void,
) {
  const getContext = createContextFactory(canvas);

  React.useEffect(function setupPanicHook() {
    if (module == null) return;
    module.setupPanicHook();
  }, [module]);


  React.useEffect(function initializeRuntime() {
    if (module == null || canvas == null) return;

    try {
      const context = getContext(canvas);
      module.start_runtime(context);
    } catch (e) {
      onError(e);
    }
  }, [canvas, module]);
}

/**
 * Creates a factory function whose result is memo'd
 */
function createContextFactory(canvas?: HTMLCanvasElement) {
  return useSingletonFactory((canvas: HTMLCanvasElement): WebGLRenderingContext => {
    return checkExists(canvas.getContext("webgl"), 'webgl is needed to run this') as WebGLRenderingContext;
  }, [canvas]);
}

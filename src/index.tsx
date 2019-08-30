import * as React from 'react';
import { render } from 'react-dom';
import "./style.css";
import { useRuntime } from 'runtime/use_runtime';
import { loadRuntime } from 'runtime/load';
import { RuntimeModule } from 'runtime/types';
import fragShader from 'shaders/frag_shader.glsl';
import vertShader from 'shaders/vert_shader.glsl';
import { Canvas } from 'ui/canvas/canvas';
import { Viewport } from 'ui/viewport/viewport';

function getRootNode() {
  const applicationRoot = document.createElement('div');
  applicationRoot.id = "application-root";
  return applicationRoot;
}

function main() {
  const runtimePromise = import("../runtime/pkg");
  const applicationRoot = getRootNode();
  document.body.appendChild(applicationRoot);

  const onError = (e: Error) => {
    console.error(e);
  };

  const useRuntimeImpl = (
      canvas: HTMLCanvasElement | undefined,
      module: RuntimeModule | undefined,
      width: number,
      height: number,
  ) => (
      useRuntime(
          canvas,
          module,
          width,
          height,
          onError,
      )
  );

  render(
      (
        <Viewport>
          <Canvas
              runtimePromise={runtimePromise}
              useRuntime={useRuntimeImpl}
          />
        </Viewport>
      ),
      applicationRoot,
  );
}

main();

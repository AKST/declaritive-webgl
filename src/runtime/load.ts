import { RuntimeModule } from './types';

export function loadRuntime(): Promise<RuntimeModule> {
  return import("../../runtime/pkg")
}

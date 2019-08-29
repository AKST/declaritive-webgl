export function checkExists<T>(value: null, message?: string): never;
export function checkExists<T>(value: undefined, message?: string): never;
export function checkExists<T>(value: T | undefined | null, message?: string): T;
export function checkExists<T>(value: T | undefined | null, message?: string): T {
  if (value == null) {
    let errorMessage = `value of "${value}" is undefined`;
    if (message != null) errorMessage += `: ${message}`;
    throw new Error(errorMessage);
  }
  return value;
}

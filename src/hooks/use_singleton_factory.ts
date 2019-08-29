import * as React from 'react';

export function useSingletonFactory<I, O>(create: (value: I) => O, changes: any[]): (value: I) => O {
  const instanceRef = React.useRef<O | undefined>();

  const getInstance = React.useCallback((input: I): O => {
    if (instanceRef.current != null) return instanceRef.current;

    const instance = create(input);
    instanceRef.current = instance;
    return instance;
  }, [instanceRef, ...changes]);

  return getInstance;
}

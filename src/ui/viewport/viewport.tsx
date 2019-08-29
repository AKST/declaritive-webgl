import * as React from 'react';
import { useMeasure } from 'ui/measure/measure';
import { ViewportDimensions } from './viewport_dimensions';
import styles from './viewport.css';

export type ViewportProps = {
  children: any;
};

export const Viewport = React.memo(function Viewport(props: ViewportProps) {
  const { children } = props;
  const [element, setElement] = React.useState<HTMLDivElement | null>(null);
  const { width, height } = useMeasure(element, window.innerWidth, window.innerHeight);
  const bounds = React.useMemo(() => ({ width, height }), [width, height]);

  return (
      <div ref={setElement} className={styles.root}>
        <ViewportDimensions.Provider value={bounds}>
          {children}
        </ViewportDimensions.Provider>
      </div>
  );
});

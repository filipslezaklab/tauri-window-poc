import './style.scss';
import { getCurrentWindow, PhysicalPosition, PhysicalSize } from '@tauri-apps/api/window';
import { useEffect, useRef } from 'react';
import { FoldSection } from './components/FoldSection';

const GAP_FROM_TASKBAR = 20; // CSS px
const HEIGHT_EPSILON = 2; // physical px

export const TrayPage = () => {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!ref.current) return;

    const win = getCurrentWindow();

    let fixedWidth: number | null = null;
    let anchorBottom: number | null = null;

    const resizeAndReposition = async () => {
      if (!ref.current) return;
      if (!(await win.isVisible())) return;

      const factor = window.devicePixelRatio;
      const rect = ref.current.getBoundingClientRect();
      const nextHeight = Math.ceil(rect.height * factor);

      const size = await win.outerSize();
      const position = await win.outerPosition();

      // Capture width ONCE — never again
      if (fixedWidth === null) {
        fixedWidth = size.width;
      }

      // Capture anchor bottom ONCE (taskbar gap)
      if (anchorBottom === null) {
        anchorBottom =
          Math.floor(screen.availHeight * factor) - GAP_FROM_TASKBAR * factor;
      }

      // If content already fits, stop
      if (Math.abs(nextHeight - size.height) <= HEIGHT_EPSILON) {
        return;
      }

      // Resize height only (width is fixed)
      await win.setSize(new PhysicalSize(fixedWidth, nextHeight));

      // Keep bottom glued to anchor
      const nextTop = anchorBottom - nextHeight;

      await win.setPosition(new PhysicalPosition(position.x, nextTop));
    };

    const ro = new ResizeObserver(() => resizeAndReposition());
    ro.observe(ref.current);

    // Initial sync
    resizeAndReposition();

    return () => {
      ro.disconnect();
    };
  }, []);

  return (
    <main id="tray-page" ref={ref}>
      <div className="folds">
        <FoldSection />
        <FoldSection />
        <FoldSection />
        <FoldSection />
      </div>
    </main>
  );
};

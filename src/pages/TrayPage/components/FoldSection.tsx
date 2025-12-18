import './style.scss';
import clsx from 'clsx';
import { useState } from 'react';

export const FoldSection = () => {
  const [open, setOpen] = useState(false);
  return (
    <div className="fold-section">
      <div className="top" onClick={() => setOpen((s) => !s)}>
        <p>Fold Title</p>
      </div>
      <div
        className={clsx('fold', {
          open,
        })}
      >
        <div className="content">
          <p>
            sssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss
          </p>
        </div>
      </div>
    </div>
  );
};

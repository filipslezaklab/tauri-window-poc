import { createFileRoute } from '@tanstack/react-router';
import { TrayPage } from '../../pages/TrayPage/TrayPage';

export const Route = createFileRoute('/_tray/tray')({
  component: TrayPage,
});

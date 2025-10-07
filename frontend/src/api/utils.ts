export function getBaseUrl(): string {
  // Check if we're in development mode
  if (import.meta.env.VITE_ENV === 'development') {
    return import.meta.env.VITE_DEV_BASE_URL || 'http://localhost:8000';
  }
  return '';
}

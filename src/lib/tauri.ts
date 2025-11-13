// Tauri API wrapper за сигурност при импортиране
declare global {
  interface Window {
    __TAURI__: any;
  }
}

// Wrapper за invoke функцията
export async function invoke<T = any>(cmd: string, args?: Record<string, any>): Promise<T> {
  if (typeof window !== 'undefined' && window.__TAURI__?.core) {
    return window.__TAURI__.core.invoke(cmd, args);
  }
  throw new Error('Tauri API not available');
}

// Wrapper за listen функцията
export async function listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<() => void> {
  if (typeof window !== 'undefined' && window.__TAURI__?.event) {
    return window.__TAURI__.event.listen(event, handler);
  }
  throw new Error('Tauri API not available');
}

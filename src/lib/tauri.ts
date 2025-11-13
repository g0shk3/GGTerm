import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { listen as tauriListen } from '@tauri-apps/api/event';

// Wrapper за invoke функцията
export async function invoke<T = any>(cmd: string, args?: Record<string, any>): Promise<T> {
  try {
    return await tauriInvoke(cmd, args);
  } catch (error) {
    console.error(`Error invoking command '${cmd}':`, error);
    throw new Error(`Tauri API invoke failed for command: ${cmd}`);
  }
}

// Wrapper за listen функцията
export async function listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<() => void> {
  try {
    return await tauriListen(event, handler);
  } catch (error) {
    console.error(`Error listening to event '${event}':`, error);
    throw new Error(`Tauri API listen failed for event: ${event}`);
  }
}

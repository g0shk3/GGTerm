export interface SSHSession {
  id: string;
  name: string;
  host: string;
  port: number;
  username: string;
  authType: 'password' | 'key';
  password?: string;
  privateKey?: string;
  group?: string;
  createdAt: string;
  updatedAt: string;
}

export interface Tab {
  id: string;
  sessionId?: string;
  title: string;
  isActive: boolean;
  isConnected: boolean;
}

export interface TerminalData {
  data: string;
}

export interface ConnectionStatus {
  tabId: string;
  connected: boolean;
  error?: string;
}

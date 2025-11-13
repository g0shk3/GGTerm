import { create } from 'zustand';
import { Tab, SSHSession } from '../types';

interface TerminalStore {
  tabs: Tab[];
  activeTabId: string | null;
  sessions: SSHSession[];

  addTab: (session?: SSHSession) => string;
  removeTab: (tabId: string) => void;
  setActiveTab: (tabId: string) => void;
  updateTabTitle: (tabId: string, title: string) => void;
  updateTabConnection: (tabId: string, isConnected: boolean) => void;

  addSession: (session: SSHSession) => void;
  updateSession: (sessionId: string, session: Partial<SSHSession>) => void;
  removeSession: (sessionId: string) => void;
  setSessions: (sessions: SSHSession[]) => void;
}

export const useTerminalStore = create<TerminalStore>((set) => ({
  tabs: [],
  activeTabId: null,
  sessions: [],

  addTab: (session) => {
    const newTab: Tab = {
      id: `tab-${Date.now()}-${Math.random()}`,
      sessionId: session?.id,
      title: session?.name || 'New Terminal',
      isActive: true,
      isConnected: false,
    };

    set((state) => ({
      tabs: [...state.tabs.map(t => ({ ...t, isActive: false })), newTab],
      activeTabId: newTab.id,
    }));

    return newTab.id;
  },

  removeTab: (tabId) => {
    set((state) => {
      const filteredTabs = state.tabs.filter(t => t.id !== tabId);
      const newActiveTab = filteredTabs.length > 0 ? filteredTabs[filteredTabs.length - 1].id : null;

      return {
        tabs: filteredTabs.map(t => ({
          ...t,
          isActive: t.id === newActiveTab,
        })),
        activeTabId: newActiveTab,
      };
    });
  },

  setActiveTab: (tabId) => {
    set((state) => ({
      tabs: state.tabs.map(t => ({
        ...t,
        isActive: t.id === tabId,
      })),
      activeTabId: tabId,
    }));
  },

  updateTabTitle: (tabId, title) => {
    set((state) => ({
      tabs: state.tabs.map(t =>
        t.id === tabId ? { ...t, title } : t
      ),
    }));
  },

  updateTabConnection: (tabId, isConnected) => {
    set((state) => ({
      tabs: state.tabs.map(t =>
        t.id === tabId ? { ...t, isConnected } : t
      ),
    }));
  },

  addSession: (session) => {
    set((state) => ({
      sessions: [...state.sessions, session],
    }));
  },

  updateSession: (sessionId, updates) => {
    set((state) => ({
      sessions: state.sessions.map(s =>
        s.id === sessionId ? { ...s, ...updates } : s
      ),
    }));
  },

  removeSession: (sessionId) => {
    set((state) => ({
      sessions: state.sessions.filter(s => s.id !== sessionId),
    }));
  },

  setSessions: (sessions) => {
    set({ sessions });
  },
}));

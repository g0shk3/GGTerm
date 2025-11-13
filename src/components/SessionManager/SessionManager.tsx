import { useState, useEffect } from 'react';
import { invoke } from '../../lib/tauri';
import { useTerminalStore } from '../../stores/useTerminalStore';
import { SSHSession } from '../../types';

interface SessionManagerProps {
  onClose: () => void;
}

export default function SessionManager({ onClose }: SessionManagerProps) {
  const { sessions, setSessions, addTab } = useTerminalStore();
  const [isCreating, setIsCreating] = useState(false);
  const [formData, setFormData] = useState({
    name: '',
    host: '',
    port: '22',
    username: '',
    authType: 'password' as 'password' | 'key',
    password: '',
    privateKey: '',
  });

  useEffect(() => {
    loadSessions();
  }, []);

  const loadSessions = async () => {
    try {
      console.log('Loading sessions...');
      const result = await invoke<SSHSession[]>('get_sessions');
      console.log('Loaded sessions:', result);
      setSessions(result);
    } catch (error) {
      console.error('Failed to load sessions:', error);
    }
  };

  const handleConnect = (session: SSHSession) => {
    console.log('Connecting to session:', session);
    addTab(session);
    onClose();
  };

  const handleSaveSession = async () => {
    try {
      const session: SSHSession = {
        id: '', // Backend ще го генерира
        name: formData.name,
        host: formData.host,
        port: parseInt(formData.port),
        username: formData.username,
        authType: formData.authType,
        password: formData.authType === 'password' ? formData.password : undefined,
        privateKey: formData.authType === 'key' ? formData.privateKey : undefined,
        createdAt: '', // Backend ще го генерира
        updatedAt: '', // Backend ще го генерира
      };

      console.log('Saving session:', session);
      const result = await invoke('save_session', { session });
      console.log('Session saved, result:', result);

      await loadSessions();
      setIsCreating(false);
      setFormData({
        name: '',
        host: '',
        port: '22',
        username: '',
        authType: 'password',
        password: '',
        privateKey: '',
      });
    } catch (error) {
      console.error('Failed to save session:', error);
      alert(`Failed to save session: ${error}`);
    }
  };

  const handleDeleteSession = async (sessionId: string) => {
    try {
      await invoke('delete_session', { sessionId });
      await loadSessions();
    } catch (error) {
      console.error('Failed to delete session:', error);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-dark-surface rounded-lg shadow-2xl w-[800px] max-h-[600px] flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-dark-highlight">
          <h2 className="text-xl font-bold text-dark-cyan">SSH Sessions</h2>
          <button
            onClick={onClose}
            className="text-dark-text hover:text-dark-red transition-colors text-2xl"
          >
            ✕
          </button>
        </div>

        <div className="flex-1 overflow-y-auto p-4">
          {!isCreating ? (
            <>
              <button
                onClick={() => setIsCreating(true)}
                className="w-full mb-4 px-4 py-3 bg-dark-cyan text-dark-bg rounded hover:bg-opacity-80 transition-colors font-medium"
              >
                + New Session
              </button>

              <div className="space-y-2">
                {sessions.map((session) => (
                  <div
                    key={session.id}
                    className="flex items-center justify-between p-4 bg-dark-bg rounded hover:bg-dark-highlight transition-colors"
                  >
                    <div className="flex-1">
                      <h3 className="font-medium text-dark-text">{session.name}</h3>
                      <p className="text-sm text-gray-400">
                        {session.username}@{session.host}:{session.port}
                      </p>
                    </div>
                    <div className="flex gap-2">
                      <button
                        onClick={() => handleConnect(session)}
                        className="px-4 py-2 bg-dark-green text-dark-bg rounded hover:bg-opacity-80 transition-colors"
                      >
                        Connect
                      </button>
                      <button
                        onClick={() => handleDeleteSession(session.id)}
                        className="px-4 py-2 bg-dark-red text-dark-bg rounded hover:bg-opacity-80 transition-colors"
                      >
                        Delete
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </>
          ) : (
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-dark-text mb-1">Name</label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  className="w-full px-3 py-2 bg-dark-bg text-dark-text rounded border border-dark-highlight focus:border-dark-cyan outline-none"
                  placeholder="My Server"
                />
              </div>

              <div className="grid grid-cols-3 gap-4">
                <div className="col-span-2">
                  <label className="block text-sm font-medium text-dark-text mb-1">Host</label>
                  <input
                    type="text"
                    value={formData.host}
                    onChange={(e) => setFormData({ ...formData, host: e.target.value })}
                    className="w-full px-3 py-2 bg-dark-bg text-dark-text rounded border border-dark-highlight focus:border-dark-cyan outline-none"
                    placeholder="example.com"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-dark-text mb-1">Port</label>
                  <input
                    type="number"
                    value={formData.port}
                    onChange={(e) => setFormData({ ...formData, port: e.target.value })}
                    className="w-full px-3 py-2 bg-dark-bg text-dark-text rounded border border-dark-highlight focus:border-dark-cyan outline-none"
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-dark-text mb-1">Username</label>
                <input
                  type="text"
                  value={formData.username}
                  onChange={(e) => setFormData({ ...formData, username: e.target.value })}
                  className="w-full px-3 py-2 bg-dark-bg text-dark-text rounded border border-dark-highlight focus:border-dark-cyan outline-none"
                  placeholder="root"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-dark-text mb-1">Auth Type</label>
                <select
                  value={formData.authType}
                  onChange={(e) => setFormData({ ...formData, authType: e.target.value as 'password' | 'key' })}
                  className="w-full px-3 py-2 bg-dark-bg text-dark-text rounded border border-dark-highlight focus:border-dark-cyan outline-none"
                >
                  <option value="password">Password</option>
                  <option value="key">Private Key</option>
                </select>
              </div>

              {formData.authType === 'password' ? (
                <div>
                  <label className="block text-sm font-medium text-dark-text mb-1">Password</label>
                  <input
                    type="password"
                    value={formData.password}
                    onChange={(e) => setFormData({ ...formData, password: e.target.value })}
                    className="w-full px-3 py-2 bg-dark-bg text-dark-text rounded border border-dark-highlight focus:border-dark-cyan outline-none"
                  />
                </div>
              ) : (
                <div>
                  <label className="block text-sm font-medium text-dark-text mb-1">Private Key Path</label>
                  <input
                    type="text"
                    value={formData.privateKey}
                    onChange={(e) => setFormData({ ...formData, privateKey: e.target.value })}
                    className="w-full px-3 py-2 bg-dark-bg text-dark-text rounded border border-dark-highlight focus:border-dark-cyan outline-none"
                    placeholder="~/.ssh/id_rsa"
                  />
                </div>
              )}

              <div className="flex gap-2 pt-4">
                <button
                  onClick={handleSaveSession}
                  className="flex-1 px-4 py-2 bg-dark-cyan text-dark-bg rounded hover:bg-opacity-80 transition-colors font-medium"
                >
                  Save
                </button>
                <button
                  onClick={() => setIsCreating(false)}
                  className="flex-1 px-4 py-2 bg-dark-highlight text-dark-text rounded hover:bg-opacity-80 transition-colors"
                >
                  Cancel
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

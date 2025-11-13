import { useState, useEffect } from 'react';
import TabBar from './components/TabBar/TabBar';
import Terminal from './components/Terminal/Terminal';
import SessionManager from './components/SessionManager/SessionManager';
import { useTerminalStore } from './stores/useTerminalStore';

function App() {
  const { tabs, addTab, updateTabConnection } = useTerminalStore();
  const [showSessionManager, setShowSessionManager] = useState(false);

  useEffect(() => {
    // Добавяме първи таб при стартиране
    if (tabs.length === 0) {
      addTab();
    }

    // Keyboard shortcuts
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey || e.metaKey) {
        if (e.key === 't') {
          e.preventDefault();
          addTab();
        } else if (e.key === 's') {
          e.preventDefault();
          setShowSessionManager(true);
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [tabs.length]);

  return (
    <div className="flex flex-col h-screen bg-dark-bg">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-2 bg-dark-surface border-b border-dark-highlight">
        <h1 className="text-lg font-bold text-dark-cyan">GGTerm</h1>
        <button
          onClick={() => setShowSessionManager(true)}
          className="px-3 py-1 bg-dark-highlight text-dark-text rounded hover:bg-dark-cyan hover:text-dark-bg transition-colors text-sm"
        >
          Sessions (Ctrl+S)
        </button>
      </div>

      {/* Tab Bar */}
      <TabBar />

      {/* Terminal Area */}
      <div className="flex-1 overflow-hidden">
        {tabs.map((tab) => (
          <div
            key={tab.id}
            className={`w-full h-full ${tab.isActive ? 'block' : 'hidden'}`}
          >
            <Terminal
              tabId={tab.id}
              sessionId={tab.sessionId}
              onConnectionChange={(connected) => updateTabConnection(tab.id, connected)}
            />
          </div>
        ))}
      </div>

      {/* Session Manager Modal */}
      {showSessionManager && (
        <SessionManager onClose={() => setShowSessionManager(false)} />
      )}
    </div>
  );
}

export default App;

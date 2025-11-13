import { useTerminalStore } from '../../stores/useTerminalStore';

export default function TabBar() {
  const { tabs, setActiveTab, removeTab, addTab } = useTerminalStore();

  const handleNewTab = () => {
    addTab();
  };

  const handleCloseTab = (tabId: string, e: React.MouseEvent) => {
    e.stopPropagation();
    removeTab(tabId);
  };

  return (
    <div className="flex items-center bg-dark-surface border-b border-dark-highlight h-10 select-none">
      <div className="flex-1 flex items-center overflow-x-auto scrollbar-thin">
        {tabs.map((tab) => (
          <div
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`
              flex items-center gap-2 px-4 py-2 min-w-[150px] max-w-[200px] cursor-pointer
              border-r border-dark-highlight transition-colors
              ${tab.isActive
                ? 'bg-dark-bg text-dark-cyan'
                : 'bg-dark-surface text-dark-text hover:bg-dark-highlight'
              }
            `}
          >
            <div className={`w-2 h-2 rounded-full ${tab.isConnected ? 'bg-dark-green' : 'bg-dark-red'}`} />
            <span className="flex-1 truncate text-sm">{tab.title}</span>
            <button
              onClick={(e) => handleCloseTab(tab.id, e)}
              className="text-dark-text hover:text-dark-red transition-colors"
              title="Close"
            >
              ✕
            </button>
          </div>
        ))}
      </div>

      <button
        onClick={handleNewTab}
        className="px-4 py-2 text-dark-cyan hover:bg-dark-highlight transition-colors"
        title="New Tab (Ctrl+T)"
      >
        +
      </button>
    </div>
  );
}

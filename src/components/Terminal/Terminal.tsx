import { useEffect, useRef } from 'react';
import { Terminal as XTerm } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebLinksAddon } from '@xterm/addon-web-links';
import '@xterm/xterm/css/xterm.css';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface TerminalProps {
  tabId: string;
  sessionId?: string;
  onConnectionChange?: (connected: boolean) => void;
}

export default function Terminal({ tabId, sessionId, onConnectionChange }: TerminalProps) {
  const terminalRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<XTerm | null>(null);
  const fitAddonRef = useRef<FitAddon | null>(null);
  const isConnectedRef = useRef(false);

  useEffect(() => {
    if (!terminalRef.current) return;

    // Създаваме терминал с тъмна тема
    const xterm = new XTerm({
      cursorBlink: true,
      cursorStyle: 'block',
      fontFamily: '"Cascadia Code", "Fira Code", "JetBrains Mono", Consolas, monospace',
      fontSize: 14,
      lineHeight: 1.2,
      theme: {
        background: '#1e1e2e',
        foreground: '#f8f8f2',
        cursor: '#f8f8f2',
        cursorAccent: '#282a36',
        selectionBackground: '#44475a',
        black: '#21222c',
        red: '#ff5555',
        green: '#50fa7b',
        yellow: '#f1fa8c',
        blue: '#bd93f9',
        magenta: '#ff79c6',
        cyan: '#8be9fd',
        white: '#f8f8f2',
        brightBlack: '#6272a4',
        brightRed: '#ff6e6e',
        brightGreen: '#69ff94',
        brightYellow: '#ffffa5',
        brightBlue: '#d6acff',
        brightMagenta: '#ff92df',
        brightCyan: '#a4ffff',
        brightWhite: '#ffffff',
      },
      allowProposedApi: true,
    });

    const fitAddon = new FitAddon();
    const webLinksAddon = new WebLinksAddon();

    xterm.loadAddon(fitAddon);
    xterm.loadAddon(webLinksAddon);

    xterm.open(terminalRef.current);
    fitAddon.fit();

    xtermRef.current = xterm;
    fitAddonRef.current = fitAddon;

    // Слушаме за промяна на размера на прозореца
    const handleResize = () => {
      fitAddon.fit();
    };
    window.addEventListener('resize', handleResize);

    // Слушаме за данни от терминала
    const unlistenData = listen<{ tab_id: string; data: string }>(
      'terminal-data',
      (event) => {
        if (event.payload.tab_id === tabId) {
          xterm.write(event.payload.data);
        }
      }
    );

    // Слушаме за статус на връзката
    const unlistenStatus = listen<{ tab_id: string; connected: boolean; error?: string }>(
      'connection-status',
      (event) => {
        if (event.payload.tab_id === tabId) {
          isConnectedRef.current = event.payload.connected;
          onConnectionChange?.(event.payload.connected);

          if (event.payload.error) {
            xterm.write(`\r\n\x1b[31mError: ${event.payload.error}\x1b[0m\r\n`);
          }
        }
      }
    );

    // Изпращаме данни към SSH сървъра
    xterm.onData((data) => {
      // Винаги изпращаме - backend ще игнорира ако няма връзка
      invoke('send_terminal_input', { tabId, data }).catch((err) => {
        console.error('Failed to send input:', err);
      });
    });

    // Welcome message
    if (!sessionId) {
      xterm.write('Welcome to GGTerm! 🚀\r\n');
      xterm.write('Select a session or create a new one to connect.\r\n\r\n');
    }

    return () => {
      window.removeEventListener('resize', handleResize);
      unlistenData.then((fn) => fn());
      unlistenStatus.then((fn) => fn());
      xterm.dispose();
    };
  }, [tabId]);

  useEffect(() => {
    if (sessionId && xtermRef.current) {
      // Свързваме се към сесията
      xtermRef.current.write('\r\nConnecting to SSH...\r\n');

      invoke('connect_ssh', { tabId, sessionId })
        .then(() => {
          isConnectedRef.current = true;
          onConnectionChange?.(true);
        })
        .catch((error) => {
          xtermRef.current?.write(`\r\n\x1b[31mConnection failed: ${error}\x1b[0m\r\n`);
          isConnectedRef.current = false;
          onConnectionChange?.(false);
        });
    }
  }, [sessionId]);

  return (
    <div className="w-full h-full bg-dark-bg">
      <div ref={terminalRef} className="w-full h-full" />
    </div>
  );
}

'use client';

import { useState, useRef, useEffect } from 'react';
import { Send, Bot, User, Play, Loader2 } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { generatePlan } from '../services/llm';

interface Message {
  role: 'user' | 'assistant' | 'system';
  content: string;
}

export default function ChatInterface() {
  const [input, setInput] = useState('');
  const [messages, setMessages] = useState<Message[]>([
    { role: 'system', content: 'OpenVy-Win ready. Type a command like "Open Notepad and type Hello".' }
  ]);
  const [isLoading, setIsLoading] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || isLoading) return;

    const userMsg = input;
    setInput('');
    setMessages(prev => [...prev, { role: 'user', content: userMsg }]);
    setIsLoading(true);

    try {
      // 1. Get Context
      setMessages(prev => [...prev, { role: 'system', content: 'Analyzing screen...' }]);
      const uiTree = await invoke('get_ui_tree');
      console.log("UI Tree:", uiTree);

      // 2. Plan with LLM
      setMessages(prev => [...prev, { role: 'system', content: 'Thinking...' }]);
      const plan = await generatePlan(userMsg, uiTree);

      setMessages(prev => [...prev, { role: 'assistant', content: `Plan: ${JSON.stringify(plan, null, 2)}` }]);

      // 3. Execute
      setMessages(prev => [...prev, { role: 'system', content: 'Executing actions...' }]);
      for (const action of plan) {
        await invoke('execute_action', {
            actionType: action.action,
            x: action.x || 0,
            y: action.y || 0,
            text: action.text
        });
        // Small delay between actions
        await new Promise(r => setTimeout(r, 500));
      }

      setMessages(prev => [...prev, { role: 'system', content: 'Done.' }]);

    } catch (error: any) {
      console.error(error);
      setMessages(prev => [...prev, { role: 'system', content: `Error: ${error.message || error}` }]);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex flex-col h-screen bg-gray-950 text-gray-100 p-4 font-sans">
      <div className="flex-1 overflow-y-auto mb-4 space-y-4 pr-2" ref={scrollRef}>
        {messages.map((m, i) => (
          <div key={i} className={`flex ${m.role === 'user' ? 'justify-end' : 'justify-start'}`}>
            <div className={`max-w-[80%] rounded-lg p-3 ${
              m.role === 'user' ? 'bg-blue-600' :
              m.role === 'assistant' ? 'bg-gray-800 border border-gray-700' :
              'bg-transparent text-gray-400 text-sm italic border border-gray-800'
            }`}>
              <div className="flex items-center gap-2 mb-1 opacity-50 text-xs uppercase tracking-wide">
                {m.role === 'user' ? <User size={12} /> : m.role === 'assistant' ? <Bot size={12} /> : <Play size={12} />}
                {m.role}
              </div>
              <pre className="whitespace-pre-wrap font-mono text-sm">{m.content}</pre>
            </div>
          </div>
        ))}
        {isLoading && (
            <div className="flex justify-start">
                <div className="bg-gray-900 rounded-lg p-3 flex items-center gap-2">
                    <Loader2 className="animate-spin text-blue-500" size={16} />
                    <span className="text-gray-400 text-sm">Processing...</span>
                </div>
            </div>
        )}
      </div>

      <div className="flex gap-2">
        <input
          className="flex-1 bg-gray-900 border border-gray-700 rounded-lg px-4 py-3 focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="What should I do?"
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={e => e.key === 'Enter' && handleSend()}
          disabled={isLoading}
        />
        <button
          onClick={handleSend}
          disabled={isLoading}
          className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 text-white rounded-lg px-4 flex items-center justify-center transition-colors"
        >
          <Send size={20} />
        </button>
      </div>
    </div>
  );
}

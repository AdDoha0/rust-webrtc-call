import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { Send, Mic, MicOff } from 'lucide-react';
import { cn } from '../../../lib/utils';
import QuickMessages from './QuickMessages';

const ChatInput = ({ onSendMessage, isCallActive }) => {
  const [message, setMessage] = useState('');
  const [isTyping, setIsTyping] = useState(false);

  const handleSubmit = (e) => {
    e.preventDefault();
    if (message.trim() && isCallActive) {
      onSendMessage(message);
      setMessage('');
      setIsTyping(false);
    }
  };

  const handleKeyPress = (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit(e);
    }
  };

  const handleInputChange = (e) => {
    setMessage(e.target.value);
    setIsTyping(e.target.value.length > 0);
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-3">
      <QuickMessages onSendMessage={onSendMessage} isCallActive={isCallActive} />
      <div className="relative">
        <textarea
          value={message}
          onChange={handleInputChange}
          onKeyPress={handleKeyPress}
          placeholder={isCallActive ? "Введите сообщение..." : "Чат недоступен во время звонка"}
          disabled={!isCallActive}
          className={cn(
            "w-full resize-none rounded-lg border border-white/20 bg-white/10 px-4 py-3 pr-12 text-white placeholder-white/50 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200",
            !isCallActive && "opacity-50 cursor-not-allowed"
          )}
          rows={1}
          style={{ minHeight: '44px', maxHeight: '120px' }}
        />
        
        <div className="absolute right-2 top-1/2 transform -translate-y-1/2 flex items-center gap-1">
          {!isCallActive && (
            <MicOff className="w-4 h-4 text-slate-500" />
          )}
          
          <motion.button
            type="submit"
            disabled={!message.trim() || !isCallActive}
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            className={cn(
              "p-2 rounded-lg transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 focus:ring-offset-slate-900",
              message.trim() && isCallActive
                ? "bg-primary-600 hover:bg-primary-700 text-white"
                : "bg-slate-600 text-slate-400 cursor-not-allowed"
            )}
          >
            <Send className="w-4 h-4" />
          </motion.button>
        </div>
      </div>

      {/* Индикатор печати */}
      {isTyping && isCallActive && (
        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          className="flex items-center gap-2 text-xs text-slate-400"
        >
          <div className="flex gap-1">
            <div className="w-1 h-1 bg-slate-400 rounded-full animate-bounce" />
            <div className="w-1 h-1 bg-slate-400 rounded-full animate-bounce" style={{ animationDelay: '0.1s' }} />
            <div className="w-1 h-1 bg-slate-400 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }} />
          </div>
          <span>Печатаете...</span>
        </motion.div>
      )}

      {/* Подсказка */}
      {!isCallActive && (
        <div className="text-xs text-slate-400 text-center">
          Чат будет доступен после начала звонка
        </div>
      )}
    </form>
  );
};

export default ChatInput; 
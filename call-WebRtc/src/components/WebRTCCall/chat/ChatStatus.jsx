import React from 'react';
import { motion } from 'framer-motion';
import { Circle, MessageSquare } from 'lucide-react';
import { cn } from '../../../lib/utils';

const ChatStatus = ({ isCallActive, unreadCount, isTyping }) => {
  return (
    <div className="flex items-center gap-2">
      <div className="flex items-center gap-1">
        <Circle className={cn(
          "w-2 h-2",
          isCallActive ? "text-success-400 fill-current" : "text-slate-500"
        )} />
        <span className="text-xs text-slate-400">
          {isCallActive ? 'Чат активен' : 'Чат неактивен'}
        </span>
      </div>
      
      {unreadCount > 0 && (
        <motion.div
          initial={{ scale: 0 }}
          animate={{ scale: 1 }}
          className="flex items-center gap-1 px-2 py-0.5 bg-primary-600/20 rounded-full"
        >
          <MessageSquare className="w-3 h-3 text-primary-400" />
          <span className="text-xs text-primary-400 font-medium">
            {unreadCount}
          </span>
        </motion.div>
      )}
      
      {isTyping && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="text-xs text-slate-400"
        >
          Печатает...
        </motion.div>
      )}
    </div>
  );
};

export default ChatStatus; 
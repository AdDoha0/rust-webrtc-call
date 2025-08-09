import React from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { X, MessageCircle } from 'lucide-react';

const ChatNotification = ({ message, onClose }) => {
  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, x: 300, scale: 0.8 }}
        animate={{ opacity: 1, x: 0, scale: 1 }}
        exit={{ opacity: 0, x: 300, scale: 0.8 }}
        transition={{ duration: 0.3 }}
        className="glass-effect p-4 rounded-lg shadow-lg max-w-sm"
      >
        <div className="flex items-start gap-3">
          <div className="w-8 h-8 bg-primary-600/20 rounded-full flex items-center justify-center flex-shrink-0">
            <MessageCircle className="w-4 h-4 text-primary-400" />
          </div>
          
          <div className="flex-1 min-w-0">
            <p className="text-sm text-white font-medium mb-1">
              Новое сообщение
            </p>
            <p className="text-xs text-slate-300 line-clamp-2">
              {message}
            </p>
          </div>
          
          <button
            onClick={onClose}
            className="p-1 hover:bg-white/10 rounded transition-colors flex-shrink-0"
          >
            <X className="w-4 h-4 text-slate-400" />
          </button>
        </div>
      </motion.div>
    </AnimatePresence>
  );
};

export default ChatNotification; 
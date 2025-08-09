import React from 'react';
import { motion } from 'framer-motion';
import { cn } from '../../../lib/utils';

const QuickMessages = ({ onSendMessage, isCallActive }) => {
  const quickMessages = [
    "Привет!",
    "Как дела?",
    "Все хорошо",
    "Спасибо!",
    "До встречи!"
  ];

  const handleQuickMessage = (message) => {
    if (isCallActive) {
      onSendMessage(message);
    }
  };

  if (!isCallActive) {
    return null;
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      className="mb-3"
    >
      <div className="flex flex-wrap gap-2">
        {quickMessages.map((message, index) => (
          <motion.button
            key={index}
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            onClick={() => handleQuickMessage(message)}
            className="px-3 py-1.5 text-xs bg-slate-700/50 hover:bg-slate-600/50 text-slate-300 rounded-full transition-colors border border-slate-600/50"
          >
            {message}
          </motion.button>
        ))}
      </div>
    </motion.div>
  );
};

export default QuickMessages; 
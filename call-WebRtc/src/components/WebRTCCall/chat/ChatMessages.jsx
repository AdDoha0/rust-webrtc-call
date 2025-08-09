import React, { useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { MessageCircle, User } from 'lucide-react';
import { cn } from '../../../lib/utils';

const ChatMessages = ({ messages }) => {
  const messagesEndRef = useRef(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  if (messages.length === 0) {
    return (
      <div className="flex-1 flex flex-col items-center justify-center p-8 text-center">
        <motion.div
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          className="w-16 h-16 bg-slate-800/50 rounded-full flex items-center justify-center mb-4"
        >
          <MessageCircle className="w-8 h-8 text-slate-400" />
        </motion.div>
        <h3 className="text-lg font-medium text-white mb-2">
          Начните общение в чате
        </h3>
        <p className="text-slate-400 text-sm">
          Сообщения появятся здесь
        </p>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto p-4 space-y-4">
      <AnimatePresence>
        {messages.map((message, index) => (
          <motion.div
            key={message.id}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
            transition={{ duration: 0.3 }}
            className={cn(
              "flex gap-3",
              message.isOwn ? "flex-row-reverse" : "flex-row"
            )}
          >
            {/* Аватар */}
            <div className={cn(
              "w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0",
              message.isOwn 
                ? "bg-primary-600" 
                : "bg-slate-600"
            )}>
              <User className="w-4 h-4 text-white" />
            </div>

            {/* Сообщение */}
            <div className={cn(
              "max-w-[70%]",
              message.isOwn ? "text-right" : "text-left"
            )}>
              <div className={cn(
                "px-4 py-2 rounded-2xl",
                message.isOwn 
                  ? "bg-primary-600 text-white" 
                  : "bg-slate-700 text-white"
              )}>
                <p className="text-sm">{message.text}</p>
              </div>
              
              <div className={cn(
                "flex items-center gap-2 mt-1 text-xs text-slate-400",
                message.isOwn ? "justify-end" : "justify-start"
              )}>
                <span className="font-medium">{message.sender}</span>
                <span>{message.timestamp}</span>
              </div>
            </div>
          </motion.div>
        ))}
      </AnimatePresence>
      
      <div ref={messagesEndRef} />
    </div>
  );
};

export default ChatMessages; 
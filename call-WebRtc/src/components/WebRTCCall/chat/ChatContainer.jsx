import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { MessageCircle, X, Users, Send } from 'lucide-react';
import ChatMessages from './ChatMessages';
import ChatInput from './ChatInput';
import ChatStatus from './ChatStatus';
import ChatNotification from './ChatNotification';
import { cn } from '../../../lib/utils';

const ChatContainer = ({ roomId, isCallActive }) => {
  const [messages, setMessages] = useState([]);
  const [isChatOpen, setIsChatOpen] = useState(true);
  const [unreadCount, setUnreadCount] = useState(0);
  const [isTyping, setIsTyping] = useState(false);
  const [notifications, setNotifications] = useState([]);

  const handleSendMessage = (messageText) => {
    if (messageText.trim()) {
      const newMessage = {
        id: Date.now(),
        text: messageText,
        sender: 'Вы',
        timestamp: new Date().toLocaleTimeString('ru-RU', { 
          hour: '2-digit', 
          minute: '2-digit' 
        }),
        isOwn: true
      };
      setMessages(prev => [...prev, newMessage]);
      
      // Здесь будет логика отправки сообщения через WebRTC
      // sendMessage(messageText);
    }
  };

  const addNotification = (message) => {
    const id = Date.now();
    setNotifications(prev => [...prev, { id, message }]);
  };

  const removeNotification = (id) => {
    setNotifications(prev => prev.filter(notification => notification.id !== id));
  };

  const toggleChat = () => {
    setIsChatOpen(!isChatOpen);
    if (!isChatOpen) {
      setUnreadCount(0); // Сбрасываем счетчик непрочитанных при открытии чата
    }
  };

  return (
    <div className="relative h-full">
      {/* Кнопка переключения чата (только на мобильных) */}
      <div className="lg:hidden fixed bottom-4 right-4 z-50">
        <motion.button
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          onClick={toggleChat}
          className="glass-effect p-3 rounded-full shadow-lg"
        >
          <MessageCircle className="w-6 h-6 text-white" />
          {unreadCount > 0 && (
            <div className="absolute -top-1 -right-1 bg-danger-500 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center">
              {unreadCount > 99 ? '99+' : unreadCount}
            </div>
          )}
        </motion.button>
      </div>

      {/* Панель чата */}
      <AnimatePresence>
        {isChatOpen && (
          <motion.div
            initial={{ opacity: 0, x: 300 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: 300 }}
            transition={{ duration: 0.3 }}
            className="glass-card h-full flex flex-col"
          >
            {/* Заголовок чата */}
            <div className="flex items-center justify-between p-4 border-b border-white/10">
              <div className="flex items-center gap-3">
                <div className="w-8 h-8 bg-primary-600/20 rounded-full flex items-center justify-center">
                  <MessageCircle className="w-4 h-4 text-primary-400" />
                </div>
                <div>
                  <h3 className="font-semibold text-white">Чат комнаты</h3>
                  <ChatStatus 
                    isCallActive={isCallActive}
                    unreadCount={unreadCount}
                    isTyping={isTyping}
                  />
                </div>
              </div>
              
              <div className="flex items-center gap-2">
                <div className="flex items-center gap-1 px-2 py-1 bg-slate-800/50 rounded-full">
                  <Users className="w-3 h-3 text-slate-400" />
                  <span className="text-xs text-slate-400">#{roomId}</span>
                </div>
                <button
                  onClick={toggleChat}
                  className="lg:hidden p-1 hover:bg-white/10 rounded transition-colors"
                >
                  <X className="w-4 h-4 text-slate-400" />
                </button>
              </div>
            </div>
            
            {/* Сообщения */}
            <div className="flex-1 overflow-hidden">
              <ChatMessages messages={messages} />
            </div>
            
            {/* Поле ввода */}
            <div className="p-4 border-t border-white/10">
              <ChatInput 
                onSendMessage={handleSendMessage}
                isCallActive={isCallActive}
              />
            </div>
          </motion.div>
        )}
      </AnimatePresence>
      
      {/* Уведомления */}
      <div className="fixed top-4 right-4 z-50 space-y-2">
        {notifications.map(notification => (
          <ChatNotification
            key={notification.id}
            message={notification.message}
            onClose={() => removeNotification(notification.id)}
          />
        ))}
      </div>
    </div>
  );
};

export default ChatContainer; 
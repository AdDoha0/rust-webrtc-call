import React, { useState } from 'react';
import ChatMessages from './ChatMessages';
import ChatInput from './ChatInput';
import ChatStatus from './ChatStatus';
import ChatNotification from './ChatNotification';
import './ChatContainer.css';

const ChatContainer = ({ roomId, isCallActive }) => {
  const [messages, setMessages] = useState([]);
  const [isChatOpen, setIsChatOpen] = useState(false);
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
    <div className={`chat-container ${isChatOpen ? 'chat-open' : ''}`}>
      <button 
        className="chat-toggle-btn"
        onClick={toggleChat}
        title={isChatOpen ? 'Скрыть чат' : 'Показать чат'}
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
          <path d="M20 2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h4l4 4 4-4h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2z"/>
        </svg>
        <span className="chat-toggle-text">
          {isChatOpen ? 'Скрыть чат' : 'Чат'}
        </span>
        {unreadCount > 0 && (
          <div className="unread-badge-toggle">
            {unreadCount > 99 ? '99+' : unreadCount}
          </div>
        )}
      </button>
      
      {isChatOpen && (
        <div className="chat-panel">
          <div className="chat-header">
            <div className="chat-header-left">
              <h3>Чат комнаты</h3>
              <ChatStatus 
                isCallActive={isCallActive}
                unreadCount={unreadCount}
                isTyping={isTyping}
              />
            </div>
            <span className="room-id">{roomId}</span>
          </div>
          
          <ChatMessages messages={messages} />
          
          <ChatInput 
            onSendMessage={handleSendMessage}
            isCallActive={isCallActive}
          />
        </div>
      )}
      
      {/* Уведомления */}
      {notifications.map(notification => (
        <ChatNotification
          key={notification.id}
          message={notification.message}
          onClose={() => removeNotification(notification.id)}
        />
      ))}
    </div>
  );
};

export default ChatContainer; 
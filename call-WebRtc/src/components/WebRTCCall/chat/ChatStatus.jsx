import React from 'react';
import './ChatStatus.css';

const ChatStatus = ({ isCallActive, unreadCount = 0, isTyping = false }) => {
  return (
    <div className="chat-status">
      {isCallActive ? (
        <div className="status-indicator active">
          <div className="status-dot"></div>
          <span>Чат активен</span>
        </div>
      ) : (
        <div className="status-indicator inactive">
          <div className="status-dot"></div>
          <span>Чат недоступен</span>
        </div>
      )}
      
      {unreadCount > 0 && (
        <div className="unread-badge">
          {unreadCount > 99 ? '99+' : unreadCount}
        </div>
      )}
      
      {isTyping && (
        <div className="typing-indicator">
          <span>Печатает...</span>
          <div className="typing-dots">
            <div className="dot"></div>
            <div className="dot"></div>
            <div className="dot"></div>
          </div>
        </div>
      )}
    </div>
  );
};

export default ChatStatus; 
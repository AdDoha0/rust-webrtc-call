import React, { useRef, useEffect } from 'react';
import './ChatMessages.css';

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
      <div className="chat-messages empty">
        <div className="empty-state">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
            <path d="M20 2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h4l4 4 4-4h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2z"/>
          </svg>
          <p>Начните общение в чате</p>
          <span>Сообщения появятся здесь</span>
        </div>
      </div>
    );
  }

  return (
    <div className="chat-messages">
      {messages.map((message) => (
        <div 
          key={message.id} 
          className={`message ${message.isOwn ? 'own' : 'other'}`}
        >
          <div className="message-content">
            <div className="message-header">
              <span className="message-sender">{message.sender}</span>
              <span className="message-time">{message.timestamp}</span>
            </div>
            <div className="message-text">{message.text}</div>
          </div>
        </div>
      ))}
      <div ref={messagesEndRef} />
    </div>
  );
};

export default ChatMessages; 
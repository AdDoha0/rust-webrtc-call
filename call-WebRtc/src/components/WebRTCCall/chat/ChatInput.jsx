import React, { useState, useRef, useEffect } from 'react';
import QuickMessages from './QuickMessages';
import './ChatInput.css';

const ChatInput = ({ onSendMessage, isCallActive }) => {
  const [message, setMessage] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const inputRef = useRef(null);

  const handleSubmit = (e) => {
    e.preventDefault();
    if (message.trim()) {
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

  useEffect(() => {
    if (isCallActive) {
      inputRef.current?.focus();
    }
  }, [isCallActive]);

  return (
    <div className="chat-input-container">
      <form onSubmit={handleSubmit} className="chat-input-form">
        <div className="input-wrapper">
          <QuickMessages onSendMessage={onSendMessage} isCallActive={isCallActive} />
          <textarea
            ref={inputRef}
            value={message}
            onChange={handleInputChange}
            onKeyPress={handleKeyPress}
            placeholder="Введите сообщение..."
            className="chat-input"
            rows="1"
            disabled={!isCallActive}
          />
          <button
            type="submit"
            className={`send-button ${message.trim() ? 'active' : ''}`}
            disabled={!message.trim() || !isCallActive}
          >
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
              <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
            </svg>
          </button>
        </div>
        {!isCallActive && (
          <div className="call-required-notice">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
            </svg>
            <span>Начните звонок для отправки сообщений</span>
          </div>
        )}
      </form>
    </div>
  );
};

export default ChatInput; 
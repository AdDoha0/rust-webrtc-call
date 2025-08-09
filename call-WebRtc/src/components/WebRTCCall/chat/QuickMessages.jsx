import React, { useState } from 'react';
import './QuickMessages.css';

const QuickMessages = ({ onSendMessage, isCallActive }) => {
  const [isOpen, setIsOpen] = useState(false);

  const quickMessages = [
    { text: 'Привет!', emoji: '👋' },
    { text: 'Как дела?', emoji: '😊' },
    { text: 'Все хорошо!', emoji: '👍' },
    { text: 'Согласен', emoji: '✅' },
    { text: 'Не согласен', emoji: '❌' },
    { text: 'Подожди', emoji: '⏳' },
    { text: 'Понял', emoji: '🤝' },
    { text: 'Спасибо!', emoji: '🙏' }
  ];

  const handleQuickMessage = (message) => {
    onSendMessage(message);
    setIsOpen(false);
  };

  const toggleQuickMessages = () => {
    setIsOpen(!isOpen);
  };

  if (!isCallActive) return null;

  return (
    <div className="quick-messages-container">
      <button
        className="quick-messages-toggle"
        onClick={toggleQuickMessages}
        title="Быстрые сообщения"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
        </svg>
      </button>
      
      {isOpen && (
        <div className="quick-messages-panel">
          <div className="quick-messages-header">
            <span>Быстрые сообщения</span>
            <button 
              className="close-quick-messages"
              onClick={toggleQuickMessages}
            >
              ×
            </button>
          </div>
          <div className="quick-messages-grid">
            {quickMessages.map((item, index) => (
              <button
                key={index}
                className="quick-message-btn"
                onClick={() => handleQuickMessage(`${item.emoji} ${item.text}`)}
              >
                <span className="quick-message-emoji">{item.emoji}</span>
                <span className="quick-message-text">{item.text}</span>
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default QuickMessages; 
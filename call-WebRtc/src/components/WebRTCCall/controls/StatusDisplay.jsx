import React from 'react';

const StatusDisplay = ({ isCallActive, isMuted }) => (
  <div className="status">
    <p>Статус: {isCallActive ? 'В звонке' : 'В комнате'}</p>
    <p>Микрофон: {isMuted ? 'Выключен' : 'Включен'}</p>
  </div>
);

export default StatusDisplay;

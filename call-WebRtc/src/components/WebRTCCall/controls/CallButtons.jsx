import React from 'react';

const CallButtons = ({
  isCallActive,
  isMuted,
  startCall,
  endCall,
  toggleMute,
  leaveRoom,
}) => (
  <div className="control-buttons">
    {!isCallActive ? (
      <button onClick={startCall} className="start-call-btn">
        Начать звонок
      </button>
    ) : (
      <>
        <button
          onClick={toggleMute}
          className={`mute-btn ${isMuted ? 'muted' : ''}`}
        >
          {isMuted ? '🔇 Включить' : '🔊 Выключить'}
        </button>
        <button onClick={endCall} className="end-call-btn">
          Завершить звонок
        </button>
      </>
    )}
    <button onClick={leaveRoom} className="leave-room-btn">
      Покинуть комнату
    </button>
  </div>
);

export default CallButtons;

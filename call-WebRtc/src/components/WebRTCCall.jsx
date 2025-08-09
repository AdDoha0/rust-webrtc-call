import React, { useState, useRef, useEffect } from 'react';
import './WebRTCCall.css';

const WebRTCCall = () => {
  const [localStream, setLocalStream] = useState(null);
  const [remoteStream, setRemoteStream] = useState(null);
  const [isCallActive, setIsCallActive] = useState(false);
  const [isMuted, setIsMuted] = useState(false);
  const [roomId, setRoomId] = useState('');
  const [isInRoom, setIsInRoom] = useState(false);

  const localAudioRef = useRef(null);
  const remoteAudioRef = useRef(null);
  const peerConnectionRef = useRef(null);

  // Конфигурация STUN серверов для WebRTC
  const configuration = {
    iceServers: [
      { urls: 'stun:stun.l.google.com:19302' },
      { urls: 'stun:stun1.l.google.com:19302' }
    ]
  };

  // Инициализация локального аудио потока
  const initializeLocalStream = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ 
        audio: true, 
        video: false 
      });
      setLocalStream(stream);
      
      if (localAudioRef.current) {
        localAudioRef.current.srcObject = stream;
      }
      
      console.log('Локальный аудио поток получен');
    } catch (error) {
      console.error('Ошибка при получении аудио потока:', error);
      alert('Не удалось получить доступ к микрофону. Проверьте разрешения.');
    }
  };

  // Создание или присоединение к комнате
  const joinRoom = async () => {
    if (!roomId.trim()) {
      alert('Введите ID комнаты');
      return;
    }

    if (!localStream) {
      await initializeLocalStream();
    }

    setIsInRoom(true);
    console.log(`Присоединился к комнате: ${roomId}`);
    
    // Здесь будет логика для подключения к другим участникам
    // Пока что просто показываем, что мы в комнате
  };

  // Начать звонок
  const startCall = () => {
    if (!localStream) {
      alert('Сначала присоединитесь к комнате');
      return;
    }
    setIsCallActive(true);
    console.log('Звонок начат');
  };

  // Завершить звонок
  const endCall = () => {
    setIsCallActive(false);
    setRemoteStream(null);
    
    if (peerConnectionRef.current) {
      peerConnectionRef.current.close();
      peerConnectionRef.current = null;
    }
    
    console.log('Звонок завершен');
  };

  // Переключить микрофон
  const toggleMute = () => {
    if (localStream) {
      const audioTrack = localStream.getAudioTracks()[0];
      if (audioTrack) {
        audioTrack.enabled = !audioTrack.enabled;
        setIsMuted(!isMuted);
      }
    }
  };

  // Покинуть комнату
  const leaveRoom = () => {
    if (isCallActive) {
      endCall();
    }
    
    if (localStream) {
      localStream.getTracks().forEach(track => track.stop());
      setLocalStream(null);
    }
    
    setIsInRoom(false);
    setRoomId('');
    console.log('Покинул комнату');
  };

  // Очистка при размонтировании компонента
  useEffect(() => {
    return () => {
      if (localStream) {
        localStream.getTracks().forEach(track => track.stop());
      }
      if (peerConnectionRef.current) {
        peerConnectionRef.current.close();
      }
    };
  }, [localStream]);

  return (
    <div className="webrtc-call">
      <div className="call-container">
        <h2>Аудиозвонок WebRTC</h2>
        
        {!isInRoom ? (
          <div className="room-join">
            <div className="input-group">
              <input
                type="text"
                placeholder="Введите ID комнаты"
                value={roomId}
                onChange={(e) => setRoomId(e.target.value)}
                className="room-input"
              />
              <button 
                onClick={joinRoom}
                className="join-btn"
              >
                Присоединиться к комнате
              </button>
            </div>
          </div>
        ) : (
          <div className="call-controls">
            <div className="room-info">
              <p>Комната: <strong>{roomId}</strong></p>
            </div>
            
            <div className="audio-controls">
              <audio 
                ref={localAudioRef} 
                autoPlay 
                muted 
                className="local-audio"
              />
              {remoteStream && (
                <audio 
                  ref={remoteAudioRef} 
                  autoPlay 
                  className="remote-audio"
                />
              )}
            </div>
            
            <div className="control-buttons">
              {!isCallActive ? (
                <button 
                  onClick={startCall}
                  className="start-call-btn"
                >
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
                  <button 
                    onClick={endCall}
                    className="end-call-btn"
                  >
                    Завершить звонок
                  </button>
                </>
              )}
              
              <button 
                onClick={leaveRoom}
                className="leave-room-btn"
              >
                Покинуть комнату
              </button>
            </div>
            
            <div className="status">
              <p>Статус: {isCallActive ? 'В звонке' : 'В комнате'}</p>
              <p>Микрофон: {isMuted ? 'Выключен' : 'Включен'}</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default WebRTCCall; 
import React, { useState } from 'react';
import RoomJoinForm from './RoomJoinForm';
import CallControls from './WebRTCCall/controls/CallControls';
import ChatContainer from './WebRTCCall/chat/ChatContainer';
import { useWebRTC } from './WebRTCCall/hooks/useWebRTC';
import './WebRTCCall.css';

const WebRTCCall = () => {
  const {
    localStream,
    remoteStream,
    isCallActive,
    isMuted,
    isInRoom,
    localAudioRef,
    remoteAudioRef,
    joinRoom,
    startCall,
    endCall,
    toggleMute,
    leaveRoom,
  } = useWebRTC();

  const [roomId, setRoomId] = useState('');

  const handleJoinRoom = async () => {
    const success = await joinRoom(roomId);
    if (!success) {
      // Ошибка в joinRoom уже обработана, можно тут что-то дополнить
    }
  };

  return (
    <div className="webrtc-call">
      <div className="call-container">
        <h2>Аудиозвонок WebRTC</h2>
        {!isInRoom ? (
          <RoomJoinForm
            roomId={roomId}
            setRoomId={setRoomId}
            joinRoom={handleJoinRoom}
          />
        ) : (
          <CallControls
            roomId={roomId}
            isCallActive={isCallActive}
            isMuted={isMuted}
            localStream={localStream}
            remoteStream={remoteStream}
            localAudioRef={localAudioRef}
            remoteAudioRef={remoteAudioRef}
            startCall={startCall}
            endCall={endCall}
            toggleMute={toggleMute}
            leaveRoom={leaveRoom}
          />
        )}
      </div>
      
      {/* Чат доступен только когда пользователь в комнате */}
      {isInRoom && (
        <ChatContainer 
          roomId={roomId}
          isCallActive={isCallActive}
        />
      )}
    </div>
  );
};

export default WebRTCCall;

import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
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
    <div className="min-h-screen">
      <AnimatePresence mode="wait">
        {!isInRoom ? (
          <motion.div
            key="join-form"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.3 }}
          >
            <RoomJoinForm
              roomId={roomId}
              setRoomId={setRoomId}
              joinRoom={handleJoinRoom}
            />
          </motion.div>
        ) : (
          <motion.div
            key="call-interface"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.3 }}
            className="grid grid-cols-1 lg:grid-cols-3 gap-6 max-w-7xl mx-auto"
          >
            {/* Основная панель звонка */}
            <div className="lg:col-span-2">
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
            </div>
            
            {/* Панель чата */}
            <div className="lg:col-span-1">
              <ChatContainer 
                roomId={roomId}
                isCallActive={isCallActive}
              />
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};

export default WebRTCCall;

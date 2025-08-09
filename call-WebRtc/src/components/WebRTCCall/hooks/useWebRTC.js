import { useEffect } from 'react';
import { useMediaStream } from './useMediaStream';
import { useCallState } from './useCallState';
import { useRoom } from './useRoom';
import { usePeerConnection } from './usePeerConnection';

export const useWebRTC = () => {
  const {
    localStream,
    localAudioRef,
    initializeLocalStream,
    toggleMute: toggleMuteStream,
    stopLocalStream
  } = useMediaStream();

  const {
    isCallActive,
    isMuted,
    remoteStream,
    remoteAudioRef,
    startCall,
    endCall,
    updateMuteState
  } = useCallState();

  const {
    isInRoom,
    roomId,
    joinRoom: joinRoomBase,
    leaveRoom: leaveRoomBase
  } = useRoom();

  const {
    closePeerConnection
  } = usePeerConnection();

  // Координация между хуками
  const joinRoom = async (roomId) => {
    if (!localStream) {
      await initializeLocalStream();
    }
    return await joinRoomBase(roomId);
  };

  const toggleMute = () => {
    const newMuteState = toggleMuteStream();
    updateMuteState(newMuteState);
  };

  const leaveRoom = () => {
    if (isCallActive) {
      endCall();
    }
    stopLocalStream();
    closePeerConnection();
    leaveRoomBase();
  };

  // Очистка при размонтировании
  useEffect(() => {
    return () => {
      stopLocalStream();
      closePeerConnection();
    };
  }, [stopLocalStream, closePeerConnection]);

  return {
    // Состояние
    localStream,
    remoteStream,
    isCallActive,
    isMuted,
    isInRoom,
    roomId,
    
    // Ссылки
    localAudioRef,
    remoteAudioRef,
    
    // Действия
    joinRoom,
    startCall,
    endCall,
    toggleMute,
    leaveRoom
  };
};
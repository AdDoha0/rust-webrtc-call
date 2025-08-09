import { useState, useRef } from 'react';

export const useCallState = () => {
  const [isCallActive, setIsCallActive] = useState(false);
  const [isMuted, setIsMuted] = useState(false);
  const [remoteStream, setRemoteStream] = useState(null);
  const remoteAudioRef = useRef(null);

  const startCall = () => {
    setIsCallActive(true);
    console.log('Звонок начат');
  };

  const endCall = () => {
    setIsCallActive(false);
    setRemoteStream(null);
    console.log('Звонок завершен');
  };

  const updateMuteState = (muted) => {
    setIsMuted(muted);
  };

  return {
    isCallActive,
    isMuted,
    remoteStream,
    remoteAudioRef,
    startCall,
    endCall,
    updateMuteState,
    setRemoteStream
  };
};
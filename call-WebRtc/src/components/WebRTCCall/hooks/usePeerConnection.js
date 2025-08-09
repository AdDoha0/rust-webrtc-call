import { useRef } from 'react';
import { WEBRTC_CONFIG } from './constants/webrtc-config';

export const usePeerConnection = () => {
  const peerConnectionRef = useRef(null);

  const createPeerConnection = () => {
    if (!peerConnectionRef.current) {
      peerConnectionRef.current = new RTCPeerConnection(WEBRTC_CONFIG);
      
      // TODO: добавить обработчики событий
      // peerConnectionRef.current.onicecandidate = ...
      // peerConnectionRef.current.onaddstream = ...
      
      console.log('RTCPeerConnection создан');
    }
    return peerConnectionRef.current;
  };

  const closePeerConnection = () => {
    if (peerConnectionRef.current) {
      peerConnectionRef.current.close();
      peerConnectionRef.current = null;
      console.log('RTCPeerConnection закрыт');
    }
  };

  return {
    peerConnectionRef,
    createPeerConnection,
    closePeerConnection
  };
};
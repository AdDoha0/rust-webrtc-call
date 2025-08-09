export const WEBRTC_CONFIG = {
    iceServers: [
      { urls: 'stun:stun.l.google.com:19302' },
      { urls: 'stun:stun1.l.google.com:19302' }
    ]
  };
  
  export const MEDIA_CONSTRAINTS = {
    audio: true,
    video: false
  };
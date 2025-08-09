import React, { useEffect } from 'react';

const LocalAudio = ({ localAudioRef, localStream }) => {
  useEffect(() => {
    if (localAudioRef.current && localStream) {
      localAudioRef.current.srcObject = localStream;
    }
  }, [localAudioRef, localStream]);

  return (
    <audio ref={localAudioRef} autoPlay muted className="local-audio" />
  );
};

export default LocalAudio;

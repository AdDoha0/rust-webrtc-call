import React, { useEffect } from 'react';

const RemoteAudio = ({ remoteAudioRef, remoteStream }) => {
  useEffect(() => {
    if (remoteAudioRef.current && remoteStream) {
      remoteAudioRef.current.srcObject = remoteStream;
    }
  }, [remoteAudioRef, remoteStream]);

  return (
    <audio ref={remoteAudioRef} autoPlay className="remote-audio" />
  );
};

export default RemoteAudio;

import React from 'react';
import LocalAudio from '../audio/LocalAudio';
import RemoteAudio from '../audio/RemoteAudio';
import CallButtons from './CallButtons';
import StatusDisplay from './StatusDisplay';

const CallControls = ({
  roomId,
  isCallActive,
  isMuted,
  localStream,
  remoteStream,
  localAudioRef,
  remoteAudioRef,
  startCall,
  endCall,
  toggleMute,
  leaveRoom,
}) => (
  <div className="call-controls">
    <div className="room-info">
      <p>
        Комната: <strong>{roomId}</strong>
      </p>
    </div>

    <div className="audio-controls">
      <LocalAudio localAudioRef={localAudioRef} localStream={localStream} />
      {remoteStream && <RemoteAudio remoteAudioRef={remoteAudioRef} remoteStream={remoteStream} />}
    </div>

    <CallButtons
      isCallActive={isCallActive}
      isMuted={isMuted}
      startCall={startCall}
      endCall={endCall}
      toggleMute={toggleMute}
      leaveRoom={leaveRoom}
    />

    <StatusDisplay isCallActive={isCallActive} isMuted={isMuted} />
  </div>
);

export default CallControls;

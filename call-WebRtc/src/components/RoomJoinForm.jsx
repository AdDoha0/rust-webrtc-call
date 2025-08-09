import React from 'react';

const RoomJoinForm = ({ roomId, setRoomId, joinRoom }) => (
  <div className="room-join">
    <div className="input-group">
      <input
        type="text"
        placeholder="Введите ID комнаты"
        value={roomId}
        onChange={(e) => setRoomId(e.target.value)}
        className="room-input"
      />
      <button onClick={joinRoom} className="join-btn">
        Присоединиться к комнате
      </button>
    </div>
  </div>
);

export default RoomJoinForm;

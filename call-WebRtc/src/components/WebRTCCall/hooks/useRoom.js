import { useState } from 'react';

export const useRoom = () => {
  const [isInRoom, setIsInRoom] = useState(false);
  const [roomId, setRoomId] = useState('');

  const joinRoom = async (roomId) => {
    if (!roomId.trim()) {
      alert('Введите ID комнаты');
      return false;
    }
    
    setIsInRoom(true);
    setRoomId(roomId);
    console.log(`Присоединился к комнате: ${roomId}`);
    
    // TODO: логика подключения к другим участникам через signaling
    return true;
  };

  const leaveRoom = () => {
    setIsInRoom(false);
    setRoomId('');
    console.log('Покинул комнату');
  };

  return {
    isInRoom,
    roomId,
    joinRoom,
    leaveRoom
  };
};
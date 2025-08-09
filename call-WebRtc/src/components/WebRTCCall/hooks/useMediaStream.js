import { useState, useRef } from 'react';
import { MEDIA_CONSTRAINTS } from './constants/webrtc-config';

export const useMediaStream = () => {
  const [localStream, setLocalStream] = useState(null);
  const localAudioRef = useRef(null);

  const initializeLocalStream = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia(MEDIA_CONSTRAINTS);
      setLocalStream(stream);
      console.log('Локальный аудио поток получен');
      return stream;
    } catch (error) {
      console.error('Ошибка при получении аудио потока:', error);
      alert('Не удалось получить доступ к микрофону. Проверьте разрешения.');
      return null;
    }
  };

  const toggleMute = () => {
    if (localStream) {
      const audioTrack = localStream.getAudioTracks()[0];
      if (audioTrack) {
        audioTrack.enabled = !audioTrack.enabled;
        return !audioTrack.enabled;
      }
    }
    return false;
  };
  
  const stopLocalStream = () => {
    if (localStream) {
      localStream.getTracks().forEach(track => track.stop());
      setLocalStream(null);
    }
  };

  return {
    localStream,
    localAudioRef,
    initializeLocalStream,
    toggleMute,
    stopLocalStream
  };
};

// Это значит, что когда ты используешь этот хук в React-компоненте, ты можешь взять:
// localStream — твой аудио/видео поток,
// localAudioRef — ссылку для работы с аудио элементом (например, чтоб проигрывать звук),
// и функции для управления:
// initializeLocalStream() — чтобы включить микрофон и получить поток,
// toggleMute() — чтобы включать/выключать микрофон,
// stopLocalStream() — чтобы остановить микрофон.
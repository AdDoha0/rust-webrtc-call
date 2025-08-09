import React from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  Phone, 
  PhoneOff, 
  Mic, 
  MicOff, 
  LogOut, 
  Users, 
  Signal,
  Volume2,
  VolumeX
} from 'lucide-react';
import { cn } from '../../../lib/utils';
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
}) => {
  return (
    <div className="space-y-6">
      {/* Заголовок и информация о комнате */}
      <motion.div
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
        className="text-center"
      >
        <div className="inline-flex items-center gap-2 bg-primary-600/20 px-4 py-2 rounded-full mb-4">
          <Phone className="w-5 h-5 text-primary-400" />
          <span className="text-primary-400 font-medium">Аудиозвонок WebRTC</span>
        </div>
        <h2 className="text-3xl font-bold text-white mb-2">
          Комната #{roomId}
        </h2>
        <p className="text-slate-400">
          {isCallActive ? 'Активный звонок' : 'Готов к звонку'}
        </p>
      </motion.div>

      {/* Основная карточка управления */}
      <motion.div
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ delay: 0.2 }}
        className="glass-card p-8"
      >
        {/* Статус подключения */}
        <StatusDisplay 
          isCallActive={isCallActive}
          isMuted={isMuted}
          hasRemoteStream={!!remoteStream}
        />

        {/* Аудио элементы */}
        <div className="mb-8">
          <audio ref={localAudioRef} autoPlay muted />
          <audio ref={remoteAudioRef} autoPlay />
        </div>

        {/* Индикаторы потоков */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-8">
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            className="p-4 bg-slate-800/50 rounded-lg border border-slate-700"
          >
            <div className="flex items-center gap-3 mb-2">
              <div className="w-3 h-3 bg-success-500 rounded-full animate-pulse" />
              <span className="text-sm font-medium text-slate-300">Ваш микрофон</span>
            </div>
            <div className="flex items-center gap-2">
              {isMuted ? (
                <MicOff className="w-4 h-4 text-danger-400" />
              ) : (
                <Mic className="w-4 h-4 text-success-400" />
              )}
              <span className="text-xs text-slate-400">
                {isMuted ? 'Отключен' : 'Активен'}
              </span>
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            className="p-4 bg-slate-800/50 rounded-lg border border-slate-700"
          >
            <div className="flex items-center gap-3 mb-2">
              <div className={cn(
                "w-3 h-3 rounded-full",
                remoteStream ? "bg-success-500 animate-pulse" : "bg-slate-500"
              )} />
              <span className="text-sm font-medium text-slate-300">Удаленный участник</span>
            </div>
            <div className="flex items-center gap-2">
              {remoteStream ? (
                <Volume2 className="w-4 h-4 text-success-400" />
              ) : (
                <VolumeX className="w-4 h-4 text-slate-500" />
              )}
              <span className="text-xs text-slate-400">
                {remoteStream ? 'Подключен' : 'Ожидание'}
              </span>
            </div>
          </motion.div>
        </div>

        {/* Кнопки управления */}
        <CallButtons
          isCallActive={isCallActive}
          isMuted={isMuted}
          onStartCall={startCall}
          onEndCall={endCall}
          onToggleMute={toggleMute}
          onLeaveRoom={leaveRoom}
        />
      </motion.div>

      {/* Информационная панель */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.4 }}
        className="glass-card p-6"
      >
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-center">
          <div className="flex flex-col items-center gap-2">
            <Signal className="w-5 h-5 text-success-400" />
            <span className="text-sm text-slate-400">Соединение</span>
            <span className="text-xs text-success-400 font-medium">Стабильное</span>
          </div>
          <div className="flex flex-col items-center gap-2">
            <Users className="w-5 h-5 text-primary-400" />
            <span className="text-sm text-slate-400">Участники</span>
            <span className="text-xs text-primary-400 font-medium">
              {remoteStream ? '2' : '1'}
            </span>
          </div>
          <div className="flex flex-col items-center gap-2">
            <Phone className="w-5 h-5 text-warning-400" />
            <span className="text-sm text-slate-400">Статус</span>
            <span className="text-xs text-warning-400 font-medium">
              {isCallActive ? 'В звонке' : 'Готов'}
            </span>
          </div>
        </div>
      </motion.div>
    </div>
  );
};

export default CallControls;

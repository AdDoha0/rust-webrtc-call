import React from 'react';
import { motion } from 'framer-motion';
import { 
  Phone, 
  PhoneOff, 
  Mic, 
  MicOff, 
  LogOut,
  Play,
  Square
} from 'lucide-react';
import { cn } from '../../../lib/utils';

const CallButtons = ({
  isCallActive,
  isMuted,
  onStartCall,
  onEndCall,
  onToggleMute,
  onLeaveRoom,
}) => {
  return (
    <div className="space-y-4">
      {/* Основные кнопки управления */}
      <div className="flex justify-center gap-4">
        {/* Кнопка микрофона */}
        <motion.button
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          onClick={onToggleMute}
          className={cn(
            "w-14 h-14 rounded-full flex items-center justify-center transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-slate-900",
            isMuted 
              ? "bg-danger-600 hover:bg-danger-700 focus:ring-danger-500" 
              : "bg-success-600 hover:bg-success-700 focus:ring-success-500"
          )}
        >
          {isMuted ? (
            <MicOff className="w-6 h-6 text-white" />
          ) : (
            <Mic className="w-6 h-6 text-white" />
          )}
        </motion.button>

        {/* Кнопка звонка */}
        <motion.button
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          onClick={isCallActive ? onEndCall : onStartCall}
          className={cn(
            "w-16 h-16 rounded-full flex items-center justify-center transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-slate-900",
            isCallActive 
              ? "bg-danger-600 hover:bg-danger-700 focus:ring-danger-500 animate-pulse-slow" 
              : "bg-primary-600 hover:bg-primary-700 focus:ring-primary-500"
          )}
        >
          {isCallActive ? (
            <PhoneOff className="w-7 h-7 text-white" />
          ) : (
            <Phone className="w-7 h-7 text-white" />
          )}
        </motion.button>
      </div>

      {/* Дополнительные кнопки */}
      <div className="flex justify-center gap-3">
        {/* Кнопка покинуть комнату */}
        <motion.button
          whileHover={{ scale: 1.02 }}
          whileTap={{ scale: 0.98 }}
          onClick={onLeaveRoom}
          className="btn-secondary flex items-center gap-2 px-4 py-2"
        >
          <LogOut className="w-4 h-4" />
          Покинуть комнату
        </motion.button>
      </div>

      {/* Индикатор состояния */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        className="text-center"
      >
        <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-slate-800/50">
          <div className={cn(
            "w-2 h-2 rounded-full",
            isCallActive ? "bg-success-500 animate-pulse" : "bg-slate-500"
          )} />
          <span className="text-xs text-slate-400">
            {isCallActive ? 'Звонок активен' : 'Готов к звонку'}
          </span>
        </div>
      </motion.div>
    </div>
  );
};

export default CallButtons;

import React from 'react';
import { motion } from 'framer-motion';
import { 
  Phone, 
  PhoneOff, 
  Mic, 
  MicOff, 
  Signal,
  Wifi,
  WifiOff
} from 'lucide-react';
import { cn } from '../../../lib/utils';

const StatusDisplay = ({ isCallActive, isMuted, hasRemoteStream }) => {
  return (
    <div className="mb-6">
      {/* Основной статус */}
      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex items-center justify-center gap-4 mb-4"
      >
        <div className="flex items-center gap-2">
          <div className={cn(
            "w-3 h-3 rounded-full",
            isCallActive ? "bg-success-500 animate-pulse" : "bg-slate-500"
          )} />
          <span className="text-sm font-medium text-slate-300">
            {isCallActive ? 'В звонке' : 'Готов к звонку'}
          </span>
        </div>
        
        <div className="w-px h-4 bg-slate-600" />
        
        <div className="flex items-center gap-2">
          {isMuted ? (
            <MicOff className="w-4 h-4 text-danger-400" />
          ) : (
            <Mic className="w-4 h-4 text-success-400" />
          )}
          <span className="text-sm text-slate-400">
            Микрофон {isMuted ? 'отключен' : 'включен'}
          </span>
        </div>
      </motion.div>

      {/* Детальная информация о соединении */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 0.2 }}
        className="grid grid-cols-1 md:grid-cols-3 gap-3"
      >
        {/* Статус звонка */}
        <div className="p-3 bg-slate-800/30 rounded-lg border border-slate-700">
          <div className="flex items-center gap-2 mb-1">
            {isCallActive ? (
              <Phone className="w-4 h-4 text-success-400" />
            ) : (
              <PhoneOff className="w-4 h-4 text-slate-500" />
            )}
            <span className="text-xs font-medium text-slate-400">Звонок</span>
          </div>
          <span className="text-sm text-slate-300">
            {isCallActive ? 'Активен' : 'Не активен'}
          </span>
        </div>

        {/* Статус микрофона */}
        <div className="p-3 bg-slate-800/30 rounded-lg border border-slate-700">
          <div className="flex items-center gap-2 mb-1">
            {isMuted ? (
              <MicOff className="w-4 h-4 text-danger-400" />
            ) : (
              <Mic className="w-4 h-4 text-success-400" />
            )}
            <span className="text-xs font-medium text-slate-400">Микрофон</span>
          </div>
          <span className="text-sm text-slate-300">
            {isMuted ? 'Отключен' : 'Включен'}
          </span>
        </div>

        {/* Статус соединения */}
        <div className="p-3 bg-slate-800/30 rounded-lg border border-slate-700">
          <div className="flex items-center gap-2 mb-1">
            {hasRemoteStream ? (
              <Wifi className="w-4 h-4 text-success-400" />
            ) : (
              <WifiOff className="w-4 h-4 text-slate-500" />
            )}
            <span className="text-xs font-medium text-slate-400">Соединение</span>
          </div>
          <span className="text-sm text-slate-300">
            {hasRemoteStream ? 'Установлено' : 'Ожидание'}
          </span>
        </div>
      </motion.div>
    </div>
  );
};

export default StatusDisplay;

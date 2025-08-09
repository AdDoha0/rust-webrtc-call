import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { Phone, Users, ArrowRight, Hash } from 'lucide-react';
import { cn } from '../lib/utils';

const RoomJoinForm = ({ roomId, setRoomId, joinRoom }) => {
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!roomId.trim()) return;
    
    setIsLoading(true);
    try {
      await joinRoom();
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.5 }}
      className="max-w-md mx-auto"
    >
      <div className="glass-card p-8">
        <div className="text-center mb-8">
          <div className="inline-flex items-center justify-center w-16 h-16 bg-primary-600/20 rounded-full mb-4">
            <Phone className="w-8 h-8 text-primary-400" />
          </div>
          <h1 className="text-2xl font-bold text-white mb-2">
            Присоединиться к звонку
          </h1>
          <p className="text-slate-400">
            Введите номер комнаты для начала аудиозвонка
          </p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label htmlFor="roomId" className="block text-sm font-medium text-slate-300 mb-2">
              Номер комнаты
            </label>
            <div className="relative">
              <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <Hash className="h-5 w-5 text-slate-400" />
              </div>
              <input
                id="roomId"
                type="text"
                value={roomId}
                onChange={(e) => setRoomId(e.target.value)}
                placeholder="Например: 12345"
                className="input-field w-full pl-10"
                required
              />
            </div>
          </div>

          <motion.button
            type="submit"
            disabled={isLoading || !roomId.trim()}
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
            className={cn(
              "w-full btn-primary flex items-center justify-center gap-2",
              (!roomId.trim() || isLoading) && "opacity-50 cursor-not-allowed"
            )}
          >
            {isLoading ? (
              <>
                <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                Подключение...
              </>
            ) : (
              <>
                <Users className="w-4 h-4" />
                Присоединиться
                <ArrowRight className="w-4 h-4" />
              </>
            )}
          </motion.button>
        </form>

        <div className="mt-6 p-4 bg-slate-800/50 rounded-lg">
          <div className="flex items-center gap-2 text-sm text-slate-400">
            <div className="w-2 h-2 bg-success-500 rounded-full animate-pulse" />
            Сервер готов к подключению
          </div>
        </div>
      </div>
    </motion.div>
  );
};

export default RoomJoinForm;

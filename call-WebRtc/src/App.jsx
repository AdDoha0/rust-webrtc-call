import React from 'react';
import { motion } from 'framer-motion';
import WebRTCCall from './components/WebRTCCall';
import './App.css';

function App() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6 }}
        className="container mx-auto px-4 py-8"
      >
        <WebRTCCall />
      </motion.div>
    </div>
  );
}

export default App;

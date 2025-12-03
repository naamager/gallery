import React from 'react';
import LoginPage from './LoginPage';

const AuthModal = ({ onClose, onLoginSuccess }) => {
  return (
    <div className="auth-modal-overlay" onClick={onClose}>
      <div className="auth-modal-content" onClick={(e) => e.stopPropagation()}> {/* Prevent clicks inside from closing modal */}
        <button className="auth-modal-close" onClick={onClose}>&times;</button>
        <LoginPage onLoginSuccess={onLoginSuccess} />
      </div>
    </div>
  );
};

export default AuthModal;

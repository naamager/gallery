import React from 'react';

const ThankYouModal = ({ onClose }) => {
  return (
    <div className="thank-you-modal-overlay">
      <div className="thank-you-modal-content">
        <h2>תודה על הזמנתך!</h2>
        <p>הפריטים שלך יטופלו בהקדם.</p>
        <button onClick={onClose} className="thank-you-close-button">
          סגור
        </button>
      </div>
    </div>
  );
};

export default ThankYouModal;

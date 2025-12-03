import React from 'react';
import { deleteArtwork } from './api'; // Import deleteArtwork instead of checkoutArtworks

const CartPage = ({ cartItems, onRemoveFromCart, onGoBack, onCheckoutSuccess }) => {
  const calculateTotal = () => {
    return cartItems.reduce((total, item) => total + item.price, 0);
  };

  const handleCheckout = async () => {
    if (cartItems.length === 0) {
      alert("הסל שלך ריק. אנא הוסף פריטים לפני התשלום.");
      return;
    }

    try {
      // Delete each artwork individually
      for (const item of cartItems) {
        await deleteArtwork(item.idArtwork);
      }
      alert("הזמנתך בוצעה בהצלחה!");
      onCheckoutSuccess(); // Notify App.js to show thank you and clear cart
    } catch (error) {
      console.error("Error during checkout:", error);
      alert("אירעה שגיאה במהלך התשלום. אנא נסה שוב.");
    }
  };

  return (
    <div className="cart-page">
      <header className="cart-header">
        <h1>סל הקניות שלך</h1>
        <button onClick={onGoBack} className="back-to-artworks-button">חזור לגלריה</button>
      </header>

      {cartItems.length === 0 ? (
        <p className="empty-cart-message">הסל שלך ריק.</p>
      ) : (
        <div className="cart-items-container">
          {cartItems.map((item) => (
            <div key={item.idArtwork} className="cart-item-card">
              {item.imageUrl && <img src={item.imageUrl} alt={item.title} className="cart-item-image" />}
              <div className="cart-item-details">
                <h2>{item.title}</h2>
                <p><strong>אמן:</strong> {item.artistName}</p>
                <p><strong>מחיר:</strong> ${item.price.toLocaleString()}</p>
                <button onClick={() => onRemoveFromCart(item.idArtwork)} className="remove-from-cart-button">
                  הסר
                </button>
              </div>
            </div>
          ))}
          <div className="cart-summary">
            <h2>סה"כ: ${calculateTotal().toLocaleString()}</h2>
            <button onClick={handleCheckout} className="checkout-button">לתשלום</button>
          </div>
        </div>
      )}
    </div>
  );
};

export default CartPage;

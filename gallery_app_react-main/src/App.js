import React, { useState } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import ArtworksPage from './ArtworksPage';
import AdminArtworksPage from './AdminArtworksPage';
import AuthModal from './AuthModal';
import CartPage from './CartPage'; // Import CartPage
import LoginPage from './LoginPage'; // Import LoginPage
import ThankYouModal from './ThankYouModal'; // Import ThankYouModal
import './App.css';
import './ArtworksPage.css';
import './LoginPage.css';
import './AuthModal.css';
import './CartPage.css'; // Import CartPage CSS
import './ThankYouModal.css'; // Import ThankYouModal CSS

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [showAuthModal, setShowAuthModal] = useState(false);
  const [cartItems, setCartItems] = useState([]); // State for cart items
  const [showCartModal, setShowCartModal] = useState(false); // State to show cart modal
  const [user, setUser] = useState(null); // New state to store user data
  const [showThankYouModal, setShowThankYouModal] = useState(false); // New state for thank you modal
  const [checkoutTrigger, setCheckoutTrigger] = useState(0); // New state to trigger artwork refresh

  const handleLoginSuccess = (userData) => {
    setIsAuthenticated(true);
    setUser(userData); // Store user data
    setShowAuthModal(false);
  };

  const handleLogout = () => {
    setIsAuthenticated(false);
    setUser(null); // Clear user data on logout
    setCartItems([]); // Clear cart on logout
  };

  const handleShowAuth = () => {
    setShowAuthModal(true);
  };

  const handleCloseAuthModal = () => {
    setShowAuthModal(false);
  };

  const handleAddToCart = (artwork) => {
    setCartItems((prevItems) => {
      const existingItem = prevItems.find(item => item.idArtwork === artwork.idArtwork);
      if (existingItem) {
        // If item already exists, for now, we'll just not add it again
        // In a real app, you might want to increment a quantity here
        return prevItems;
      } else {
        return [...prevItems, { ...artwork, quantity: 1 }];
      }
    });
  };

  const handleRemoveFromCart = (idArtwork) => {
    setCartItems((prevItems) => prevItems.filter(item => item.idArtwork !== idArtwork));
  };

  const handleShowCart = () => {
    setShowCartModal(true);
  };

  const handleCloseCartModal = () => {
    setShowCartModal(false);
  };

  const handleCheckoutSuccess = () => {
    setCartItems([]); // Clear the cart
    setShowCartModal(false); // Close the cart modal
    setShowThankYouModal(true); // Show the thank you modal
    setCheckoutTrigger(prev => prev + 1); // Trigger artwork refresh
  };

  const handleCloseThankYouModal = () => {
    setShowThankYouModal(false);
  };

  return (
    <Router>
      <div className="App">
        <Routes>
          <Route path="/" element={
            <ArtworksPage
              isAuthenticated={isAuthenticated}
              onShowAuth={handleShowAuth}
              onLogout={handleLogout}
              onAddToCart={handleAddToCart}
              cartItemCount={cartItems.length}
              onShowCart={handleShowCart}
              userName={user ? user.first_name : ''}
              checkoutTrigger={checkoutTrigger}
              cartItems={cartItems} // Pass cartItems to ArtworksPage
            />
          } />
          <Route path="/admin/artworks" element={<AdminArtworksPage />} />
          <Route path="/login" element={<LoginPage onLoginSuccess={handleLoginSuccess} />} />
        </Routes>

        {showAuthModal && (
          <AuthModal
            onClose={handleCloseAuthModal}
            onLoginSuccess={handleLoginSuccess}
          />
        )}

        {showCartModal && (
          <div className="cart-modal-overlay">
            <div className="cart-modal-content">
              <CartPage
                cartItems={cartItems}
                onRemoveFromCart={handleRemoveFromCart}
                onGoBack={handleCloseCartModal} // Use onClose for the modal
                onCheckoutSuccess={handleCheckoutSuccess} // Pass new callback
              />
            </div>
          </div>
        )}

        {showThankYouModal && (
          <ThankYouModal onClose={handleCloseThankYouModal} />
        )}
      </div>
    </Router>
  );
}

export default App;

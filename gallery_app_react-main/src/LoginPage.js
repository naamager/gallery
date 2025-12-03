import React, { useState } from 'react';
import { createCustomer, loginCustomer } from './api';

const LoginPage = ({ onLoginSuccess }) => {
  const [isLogin, setIsLogin] = useState(true); // State to toggle between login and registration
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');
  const [phone, setPhone] = useState('');
  const [address, setAddress] = useState('');

  const handleLoginSubmit = async (e) => {
    e.preventDefault();
    try {
      const customer = await loginCustomer(email, password);
      console.log('Login successful:', customer);
      onLoginSuccess(customer); // Call the success handler from App.js with customer data
      setEmail(''); // Clear email and password fields
      setPassword('');
    } catch (error) {
      console.error("Error during login:", error);
      console.error("Error message:", error.message);
      if (error.message.includes("Customer not found or invalid credentials") || error.message.includes("Invalid credentials")) {
        alert('שם משתמש או סיסמה שגויים.');
      } else {
        alert("ההתחברות נכשלה. אנא נסה שוב מאוחר יותר.");
      }
    }
  };

  const handleRegisterSubmit = async (e) => {
    e.preventDefault();
    try {
      const customerData = {
        customer_id: "CUST_TEMP_001", // Added temporary customer_id for testing
        first_name: firstName,
        last_name: lastName,
        phone,
        email,
        address,
        password,
      };

      const newCustomer = await createCustomer(customerData);
      console.log('Registration successful:', newCustomer);
      alert('הרשמה בוצעה בהצלחה! אנא התחבר.');
      setIsLogin(true); // Switch to login form after successful registration
      setEmail(''); // Clear email and password fields
      setPassword('');
      setFirstName('');
      setLastName('');
      setPhone('');
      setAddress('');
    } catch (error) {
      console.error("Error during registration:", error);
      console.error("Error message:", error.message);
      alert("שגיאה בהרשמה. אנא ודא שכל הפרטים נכונים ונסה שוב.");
    }
  };

  return (
    <div className="login-page">
      <div className="auth-container">
        <div className="auth-toggle">
          <button
            className={isLogin ? 'active' : ''}
            onClick={() => setIsLogin(true)}
          >
            התחברות
          </button>
          <button
            className={!isLogin ? 'active' : ''}
            onClick={() => setIsLogin(false)}
          >
            הרשמה
          </button>
        </div>

        {isLogin ? (
          <form onSubmit={handleLoginSubmit} className="auth-form">
            <h2>התחברות</h2>
            <div className="form-group">
              <label htmlFor="loginEmail">אימייל:</label>
              <input
                type="email"
                id="loginEmail"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
              />
            </div>
            <div className="form-group">
              <label htmlFor="loginPassword">סיסמה:</label>
              <input
                type="password"
                id="loginPassword"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
              />
            </div>
            <button type="submit" className="submit-button">התחבר</button>
          </form>
        ) : (
          <form onSubmit={handleRegisterSubmit} className="auth-form">
            <h2>הרשמה</h2>
            <div className="form-group">
              <label htmlFor="firstName">שם פרטי:</label>
              <input
                type="text"
                id="firstName"
                value={firstName}
                onChange={(e) => setFirstName(e.target.value)}
                required
              />
            </div>
            <div className="form-group">
              <label htmlFor="lastName">שם משפחה:</label>
              <input
                type="text"
                id="lastName"
                value={lastName}
                onChange={(e) => setLastName(e.target.value)}
                required
              />
            </div>
            <div className="form-group">
              <label htmlFor="phone">טלפון:</label>
              <input
                type="tel"
                id="phone"
                value={phone}
                onChange={(e) => setPhone(e.target.value)}
                required
              />
            </div>
            <div className="form-group">
              <label htmlFor="registerEmail">אימייל:</label>
              <input
                type="email"
                id="registerEmail"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
              />
            </div>
            <div className="form-group">
              <label htmlFor="address">כתובת:</label>
              <input
                type="text"
                id="address"
                value={address}
                onChange={(e) => setAddress(e.target.value)}
                required
              />
            </div>
            <div className="form-group">
              <label htmlFor="registerPassword">סיסמה:</label>
              <input
                type="password"
                id="registerPassword"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
              />
            </div>
            <button type="submit" className="submit-button">הירשם</button>
          </form>
        )}
      </div>
    </div>
  );
};

export default LoginPage;

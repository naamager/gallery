import React, { useState, useEffect } from 'react';
import { getArtworks, getArtists } from './api';

// Helper function to convert snake_case to camelCase
const snakeToCamel = (obj) => {
  if (Array.isArray(obj)) {
    return obj.map(v => snakeToCamel(v));
  } else if (obj !== null && typeof obj === 'object') {
    return Object.keys(obj).reduce((acc, key) => {
      let camelKey = key.replace(/_([a-z])/g, (g) => g[1].toUpperCase());

      // Special handling for 'id_artist' to ensure it becomes 'artistId'
      if (key === 'id_artist') {
        camelKey = 'artistId';
      }
      acc[camelKey] = snakeToCamel(obj[key]);
      return acc;
    }, {});
  } else {
    return obj;
  }
};

const ArtworksPage = ({ isAuthenticated, onShowAuth, onLogout, onAddToCart, cartItemCount, onShowCart, userName, checkoutTrigger, cartItems }) => {
  const [artworks, setArtworks] = useState([]);
  const [filteredArtworks, setFilteredArtworks] = useState([]);
  const [artistFilter, setArtistFilter] = useState('');
  const [artTypeFilter, setArtTypeFilter] = useState('');
  const [minPriceFilter, setMinPriceFilter] = useState('');
  const [maxPriceFilter, setMaxPriceFilter] = useState('');
  const [loading, setLoading] = useState(true); // New loading state
  const [error, setError] = useState(null); // New error state
  const [artistIdToNameMap, setArtistIdToNameMap] = useState({}); // New state for artist map

  // Fetch data from Rust server on component mount
  useEffect(() => {
    console.log("ArtworksPage useEffect triggered. checkoutTrigger:", checkoutTrigger);
    const fetchInitialData = async () => {
      console.log("fetchInitialData started.");
      try {
        const [artworksRawData, artistsRawData] = await Promise.all([
          getArtworks(),
          getArtists()
        ]);
        console.log("Raw artworks data after fetch:", artworksRawData);
        console.log("Raw artists data after fetch:", artistsRawData);

        // Process artists data
        const artistsCamelCase = artistsRawData.map(artist => snakeToCamel(artist));
        const artistMap = artistsCamelCase.reduce((map, artist) => {
          const camelCaseArtist = snakeToCamel(artist);
          map[camelCaseArtist.artistId] = `${camelCaseArtist.firstName} ${camelCaseArtist.lastName}`;
          return map;
        }, {});
        setArtistIdToNameMap(artistMap);
        console.log("Artist map created:", artistMap);

        // Process artworks data
        const processedArtworks = artworksRawData.map(artwork => {
          const camelCaseArtwork = snakeToCamel(artwork);
          const artistNameFromMap = artistMap[camelCaseArtwork.artistId];
          return {
            ...camelCaseArtwork,
            artistName: artistNameFromMap || 'Unknown Artist'
          };
        });
        console.log("Processed artworks before setting state:", processedArtworks);
        setArtworks(processedArtworks);
        setFilteredArtworks(processedArtworks);
        console.log("Artworks and filtered artworks state updated.");
      } catch (error) {
        console.error("Error fetching initial data:", error);
        setError("Failed to fetch artworks or artists. Please try again later.");
      } finally {
        setLoading(false);
        console.log("fetchInitialData finished. Loading set to false.");
      }
    };

    fetchInitialData();
  }, [checkoutTrigger]); // Add checkoutTrigger to dependencies

  // Apply filters whenever filter states or artworks change
  useEffect(() => {
    let currentFiltered = artworks;

    if (artistFilter) {
      currentFiltered = currentFiltered.filter(artwork =>
        artwork.artistName.toLowerCase().includes(artistFilter.toLowerCase())
      );
    }

    if (artTypeFilter) {
      currentFiltered = currentFiltered.filter(artwork =>
        artwork.artType.toLowerCase() === artTypeFilter.toLowerCase()
      );
    }

    if (minPriceFilter) {
      currentFiltered = currentFiltered.filter(artwork =>
        artwork.price >= parseFloat(minPriceFilter)
      );
    }

    if (maxPriceFilter) {
      currentFiltered = currentFiltered.filter(artwork =>
        artwork.price <= parseFloat(maxPriceFilter)
      );
    }

    setFilteredArtworks(currentFiltered);
  }, [artworks, artistFilter, artTypeFilter, minPriceFilter, maxPriceFilter]);

  // Extract unique art types for the dropdown
  const uniqueArtTypes = [...new Set(artworks.map(artwork => artwork.artType))];

  const handleAddToCart = (artwork) => {
    if (!isAuthenticated) {
      onShowAuth();
      alert('אנא התחבר או הירשם כדי להוסיף פריטים לסל.');
    } else {
      onAddToCart(artwork); // Call the prop function to add to global cart
      alert(`הפריט "${artwork.title}" נוסף לסל בהצלחה!`);
    }
  };

  const isArtworkInCart = (artworkId) => {
    return cartItems.some(item => item.idArtwork === artworkId);
  };

  return (
    <div className="artworks-page">
      <header className="artworks-header">
        <h1>גלריית יצירות אומנות</h1>
        <div className="header-buttons">
          {isAuthenticated ? (
            <>
              <span>שלום, {userName}!</span>
              <button onClick={onLogout} className="logout-button">התנתק</button>
            </>
          ) : (
            <button onClick={onShowAuth} className="auth-button">התחברות / הרשמה</button>
          )}
          <button onClick={onShowCart} className="cart-icon-button">
            🛒
            {cartItemCount > 0 && <span className="cart-item-count">{cartItemCount}</span>}
          </button>
        </div>
      </header>

      {loading && <p className="loading-message">טוען יצירות אומנות...</p>}
      {error && <p className="error-message">{error}</p>}

      {!loading && !error && (
        <div className="filters-container">
          <div className="filter-group">
            <label htmlFor="artistFilter">סינון לפי אמן:</label>
            <input
              type="text"
              id="artistFilter"
              placeholder="הכנס שם אמן"
              value={artistFilter}
              onChange={(e) => setArtistFilter(e.target.value)}
            />
          </div>

          <div className="filter-group">
            <label htmlFor="artTypeFilter">סינון לפי סוג אומנות:</label>
            <select
              id="artTypeFilter"
              value={artTypeFilter}
              onChange={(e) => setArtTypeFilter(e.target.value)}
            >
              <option value="">כל הסוגים</option>
              {uniqueArtTypes.map(type => (
                <option key={type} value={type}>{type}</option>
              ))}
            </select>
          </div>

          <div className="filter-group price-range">
            <label>סינון לפי מחיר:</label>
            <input
              type="number"
              id="minPriceFilter"
              placeholder="מחיר ממינימלי"
              value={minPriceFilter}
              onChange={(e) => setMinPriceFilter(e.target.value)}
            />
            <span>-</span>
            <input
              type="number"
              id="maxPriceFilter"
              placeholder="מחיר מקסימלי"
              value={maxPriceFilter}
              onChange={(e) => setMaxPriceFilter(e.target.value)}
            />
          </div>
        </div>
      )}

      <div className="artworks-list">
        {loading ? (
          <p>טוען יצירות אומנות...</p>
        ) : error ? (
          <p style={{ color: 'red' }}>{error}</p>
        ) : filteredArtworks.length > 0 ? (
          filteredArtworks.map((artwork) => {
            const inCart = isArtworkInCart(artwork.idArtwork);
            return (
              <div key={artwork.idArtwork} className="artwork-card">
                {artwork.imageUrl && <img src={artwork.imageUrl} alt={artwork.title} className="artwork-image" />}
                <div className="artwork-details">
                  <h2>{artwork.title}</h2>
                  <p><strong>אמן:</strong> {artwork.artistName}</p>
                  <p><strong>סוג:</strong> {artwork.artType}</p>
                  <p><strong>שנה:</strong> {artwork.yearCreated}</p>
                  <p><strong>תיאור:</strong> {artwork.description}</p>
                  <p className="artwork-price"><strong>מחיר:</strong> ${artwork.price.toLocaleString()}</p>
                  <button
                    className="add-to-cart-button"
                    onClick={() => handleAddToCart(artwork)}
                    disabled={inCart}
                  >
                    {inCart ? 'בסל' : 'הוספה לסל'}
                  </button>
                </div>
              </div>
            );
          })
        ) : (
          <p>לא נמצאו יצירות אומנות התואמות לסינון.</p>
        )}
      </div>
    </div>
  );
};

export default ArtworksPage;

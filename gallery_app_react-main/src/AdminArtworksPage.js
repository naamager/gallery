import React, { useState, useEffect } from 'react';
import { getArtworks, createArtwork, updateArtwork, deleteArtwork } from './api';

const AdminArtworksPage = () => {
  const [artworks, setArtworks] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  // State for new artwork form
  const [newArtwork, setNewArtwork] = useState({
    title: '',
    description: '',
    yearCreated: '',
    price: '',
    idArtist: '',
    artistName: '',
    artType: '',
    imageUrl: '',
  });

  // State for editing artwork
  const [editingArtwork, setEditingArtwork] = useState(null); // Stores the artwork being edited

  const fetchArtworks = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await getArtworks();
      setArtworks(data);
    } catch (error) {
      console.error("Error fetching artworks:", error);
      setError("Failed to fetch artworks. Please try again later.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchArtworks();
  }, []);

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setNewArtwork({ ...newArtwork, [name]: value });
  };

  const handleSubmitNewArtwork = async (e) => {
    e.preventDefault();
    try {
      const addedArtwork = await createArtwork({
        ...newArtwork,
        yearCreated: parseInt(newArtwork.yearCreated),
        price: parseFloat(newArtwork.price),
      });

      setArtworks([...artworks, addedArtwork]);
      setNewArtwork({
        title: '',
        description: '',
        yearCreated: '',
        price: '',
        idArtist: '',
        artistName: '',
        artType: '',
        imageUrl: '',
      });
      alert('יצירת אומנות נוספה בהצלחה!');
      fetchArtworks(); // Refresh the list of artworks
    } catch (error) {
      console.error("Error adding artwork:", error);
      alert("שגיאה בהוספת יצירת אומנות. אנא נסה שוב.");
    }
  };

  const handleEditClick = (artwork) => {
    setEditingArtwork({ ...artwork }); // Set the artwork to be edited
  };

  const handleEditFormChange = (e) => {
    const { name, value } = e.target;
    setEditingArtwork({ ...editingArtwork, [name]: value });
  };

  const handleUpdateArtwork = async (e) => {
    e.preventDefault();
    if (!editingArtwork) return;

    try {
      const updatedArtwork = await updateArtwork(editingArtwork.idArtwork, {
        ...editingArtwork,
        yearCreated: parseInt(editingArtwork.yearCreated),
        price: parseFloat(editingArtwork.price),
      });

      setArtworks(artworks.map(art => (art.idArtwork === updatedArtwork.idArtwork ? updatedArtwork : art)));
      setEditingArtwork(null); // Clear editing state
      alert('יצירת אומנות עודכנה בהצלחה!');
      fetchArtworks(); // Refresh the list of artworks
    } catch (error) {
      console.error("Error updating artwork:", error);
      alert("שגיאה בעדכון יצירת אומנות. אנא נסה שוב.");
    }
  };

  const handleDeleteArtwork = async (idArtwork) => {
    if (window.confirm('האם אתה בטוח שברצונך למחוק יצירת אומנות זו?')) {
      try {
        await deleteArtwork(idArtwork);

        setArtworks(artworks.filter(artwork => artwork.idArtwork !== idArtwork));
        alert('יצירת אומנות נמחקה בהצלחה!');
        fetchArtworks(); // Refresh the list of artworks
      } catch (error) {
        console.error("Error deleting artwork:", error);
        alert("שגיאה במחיקת יצירת אומנות. אנא נסה שוב.");
      }
    }
  };

  return (
    <div className="admin-artworks-page">
      <h1>ניהול יצירות אומנות</h1>

      <div className="add-artwork-section">
        <h2>הוסף יצירת אומנות חדשה</h2>
        <form onSubmit={handleSubmitNewArtwork}>
          <input type="text" name="title" placeholder="כותרת" value={newArtwork.title} onChange={handleInputChange} required />
          <textarea name="description" placeholder="תיאור" value={newArtwork.description} onChange={handleInputChange} required />
          <input type="number" name="yearCreated" placeholder="שנת יצירה" value={newArtwork.yearCreated} onChange={handleInputChange} required />
          <input type="number" name="price" placeholder="מחיר" value={newArtwork.price} onChange={handleInputChange} required />
          <input type="text" name="idArtist" placeholder="מזהה אמן" value={newArtwork.idArtist} onChange={handleInputChange} required />
          <input type="text" name="artistName" placeholder="שם אמן" value={newArtwork.artistName} onChange={handleInputChange} required />
          <input type="text" name="artType" placeholder="סוג אומנות" value={newArtwork.artType} onChange={handleInputChange} required />
          <input type="text" name="imageUrl" placeholder="כתובת URL לתמונה" value={newArtwork.imageUrl} onChange={handleInputChange} />
          <button type="submit">הוסף יצירת אומנות</button>
        </form>
      </div>

      {editingArtwork && (
        <div className="edit-artwork-section">
          <h2>ערוך יצירת אומנות</h2>
          <form onSubmit={handleUpdateArtwork}>
            <input type="text" name="title" placeholder="כותרת" value={editingArtwork.title} onChange={handleEditFormChange} required />
            <textarea name="description" placeholder="תיאור" value={editingArtwork.description} onChange={handleEditFormChange} required />
            <input type="number" name="yearCreated" placeholder="שנת יצירה" value={editingArtwork.yearCreated} onChange={handleEditFormChange} required />
            <input type="number" name="price" placeholder="מחיר" value={editingArtwork.price} onChange={handleEditFormChange} required />
            <input type="text" name="idArtist" placeholder="מזהה אמן" value={editingArtwork.idArtist} onChange={handleEditFormChange} required />
            <input type="text" name="artistName" placeholder="שם אמן" value={editingArtwork.artistName} onChange={handleEditFormChange} required />
            <input type="text" name="artType" placeholder="סוג אומנות" value={editingArtwork.artType} onChange={handleEditFormChange} required />
            <input type="text" name="imageUrl" placeholder="כתובת URL לתמונה" value={editingArtwork.imageUrl} onChange={handleEditFormChange} />
            <button type="submit">עדכן יצירת אומנות</button>
            <button type="button" onClick={() => setEditingArtwork(null)}>בטל</button>
          </form>
        </div>
      )}

      {loading && <p>טוען יצירות אומנות...</p>}
      {error && <p style={{ color: 'red' }}>{error}</p>}
      {!loading && !error && (
        <div className="artworks-management">
          <h2>יצירות אומנות קיימות</h2>
          {artworks.length > 0 ? (
            <ul>
              {artworks.map(artwork => (
                <li key={artwork.idArtwork}>
                  {artwork.title} by {artwork.artistName} ({artwork.yearCreated}) - ${artwork.price.toLocaleString()}
                  <button onClick={() => handleEditClick(artwork)}>ערוך</button>
                  <button onClick={() => handleDeleteArtwork(artwork.idArtwork)}>מחק</button>
                </li>
              ))}
            </ul>
          ) : (
            <p>אין יצירות אומנות להצגה.</p>
          )}
        </div>
      )}
    </div>
  );
};

export default AdminArtworksPage;

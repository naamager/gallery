const API_BASE_URL = 'http://127.0.0.1:3007';

export const getArtworks = async () => {
  const response = await fetch(`${API_BASE_URL}/artworks/`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const data = await response.json();
  console.log("Raw API response for getArtworks:", data);
  return data;
};

export const createArtwork = async (artworkData) => {
  const response = await fetch(`${API_BASE_URL}/artworks/`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(artworkData),
  });
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

export const updateArtwork = async (idArtwork, artworkData) => {
  const response = await fetch(`${API_BASE_URL}/artworks/${idArtwork}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(artworkData),
  });
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

export const deleteArtwork = async (idArtwork) => {
  const response = await fetch(`${API_BASE_URL}/artworks/${idArtwork}`, {
    method: 'DELETE',
  });
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  // Expect text response from backend for delete success
  return response.text();
};

export const createCustomer = async (customerData) => {
  const response = await fetch(`${API_BASE_URL}/customers/`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(customerData),
  });
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

export const loginCustomer = async (email, password) => {
  const params = new URLSearchParams({
    email,
    password,
  }).toString();
  const loginUrl = `${API_BASE_URL}/customers/login?${params}`;
  console.log("Attempting login to URL:", loginUrl); // Log the full URL
  const response = await fetch(loginUrl);
  if (!response.ok) {
    // The server returns a specific status code for invalid credentials or not found
    let errorMessage = `HTTP error! status: ${response.status}`;
    try {
      const errorText = await response.text();
      if (errorText) {
        errorMessage = errorText; // Use the specific error message from the server if available
      } else {
        errorMessage = `HTTP error! status: ${response.status} - No detailed error message from server.`;
      }
    } catch (e) {
      console.error("Failed to read error response text:", e);
      errorMessage = `HTTP error! status: ${response.status} - Failed to parse error response.`;
    }

    console.error("Login API error message before throwing:", errorMessage); // Added for debugging
    throw new Error(errorMessage);
  }
  return response.json();
};

export const getArtists = async () => {
  const response = await fetch(`${API_BASE_URL}/artists/`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const data = await response.json();
  console.log("Raw API response for getArtists:", data);
  return data;
};

// The checkoutArtworks function is no longer needed as we'll be deleting items individually.
// export const checkoutArtworks = async (artworkIds) => {
//   const queryParams = new URLSearchParams();
//   artworkIds.forEach(id => queryParams.append('artwork_ids', id));

//   const response = await fetch(`${API_BASE_URL}/artworks/bulk?${queryParams.toString()}`, {
//     method: 'DELETE',
//     headers: {
//       'Content-Type': 'application/json',
//     },
//     // DELETE requests typically do not have a body
//     // body: JSON.stringify({ artwork_ids: artworkIds }),
//   });
//   if (!response.ok) {
//     const errorText = await response.text();
//     console.error(`Checkout API error: HTTP status ${response.status}, message: ${errorText}`);
//     throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
//   }
//   return response.json();
// };
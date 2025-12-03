#!/bin/bash

echo "🚀 Starting the Rust SQLite CRUD app..."
echo ""

# Start the server in background
cargo run &
SERVER_PID=$!

# Wait for server to start
echo "⏳ Waiting for server to start..."
sleep 5

echo "🧪 Testing API endpoints..."
echo ""

# Test 1: Create an item
echo "1️⃣ Creating an item..."
CREATE_RESPONSE=$(curl -s -X POST http://127.0.0.1:3003/items \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Item 1"}')
echo "Response: $CREATE_RESPONSE"
echo ""

# Extract ID from response (assuming JSON response)
ITEM_ID=$(echo $CREATE_RESPONSE | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
echo "📝 Created item with ID: $ITEM_ID"
echo ""

# Test 2: Get all items
echo "2️⃣ Getting all items..."
curl -s http://127.0.0.1:3003/items | python3 -m json.tool
echo ""

# Test 3: Get specific item
echo "3️⃣ Getting item by ID..."
curl -s http://127.0.0.1:3003/items/$ITEM_ID | python3 -m json.tool
echo ""

# Test 4: Update item
echo "4️⃣ Updating item..."
UPDATE_RESPONSE=$(curl -s -X PUT http://127.0.0.1:3003/items/$ITEM_ID \
  -H "Content-Type: application/json" \
  -d '{"name": "Updated Test Item"}')
echo "Response: $UPDATE_RESPONSE"
echo ""

# Test 5: Get updated item
echo "5️⃣ Getting updated item..."
curl -s http://127.0.0.1:3003/items/$ITEM_ID | python3 -m json.tool
echo ""

# Test 6: Delete item
echo "6️⃣ Deleting item..."
DELETE_RESPONSE=$(curl -s -X DELETE http://127.0.0.1:3003/items/$ITEM_ID)
echo "Response: $DELETE_RESPONSE"
echo ""

# Test 7: Verify deletion
echo "7️⃣ Verifying item was deleted..."
curl -s http://127.0.0.1:3003/items/$ITEM_ID
echo ""

echo "✅ All tests completed!"
echo ""

# Stop the server
echo "🛑 Stopping server..."
kill $SERVER_PID 2>/dev/null
echo "Server stopped."

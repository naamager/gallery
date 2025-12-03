# SQLite CRUD App

A Rust web application using Actix Web and SQLite for CRUD operations.

## Features

- Create items
- Read items (all or by ID)
- Update items
- Delete items
- SQLite database with automatic schema creation

## API Endpoints

- `POST /items` - Create a new item
- `GET /items` - Get all items
- `GET /items/{id}` - Get item by ID
- `PUT /items/{id}` - Update item by ID
- `DELETE /items/{id}` - Delete item by ID

## Running the App

```bash
cargo run
```

The server will start on `http://127.0.0.1:3003`

## Database

The app uses an in-memory SQLite database with the following schema:

```sql
CREATE TABLE items (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

Note: Data is stored in memory and will be lost when the server restarts. For persistent storage, change the database URL in `src/main.rs` from `"sqlite::memory:"` to `"sqlite:app.db"`.

## Example Usage

### Create an item
```bash
curl -X POST http://127.0.0.1:3003/items \
  -H "Content-Type: application/json" \
  -d '{"name": "My Item"}'
```

### Get all items
```bash
curl http://127.0.0.1:3003/items
```

### Update an item
```bash
curl -X PUT http://127.0.0.1:3003/items/{id} \
  -H "Content-Type: application/json" \
  -d '{"name": "Updated Item"}'
```

### Delete an item
```bash
curl -X DELETE http://127.0.0.1:3003/items/{id}
```

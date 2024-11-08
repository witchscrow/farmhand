# silo

Silo is the API service component of Farmhand that handles file uploads, user management, and data operations. Think of it as the storage barn for all incoming content.

## Technical Overview
- **Runtime**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL
- **Authentication**: JWT-based
- **File Handling**: Multipart uploads with temp storage

## Features
- 📤 File upload management
- 👤 User authentication and authorization
- 🗄️ Content metadata management
- 📡 WebSocket support for real-time updates
- 🔍 Content search and filtering
- 📊 System metrics and logging

## Getting Started

```bash
cargo run
```

## Environment Variables
```env
DATABASE_URL=     # PostgreSQL connection string
JWT_SECRET=       # Secret for JWT token generation
UPLOAD_DIR=       # Directory for temporary file storage
```

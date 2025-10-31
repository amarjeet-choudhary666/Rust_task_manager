# 🦀 Rust Backend API

A RESTful API built with Rust using Actix-Web framework, providing user authentication and task management functionality with MongoDB as the database.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/actix%20web-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://actix.rs/)
[![MongoDB](https://img.shields.io/badge/MongoDB-%234ea94b.svg?style=for-the-badge&logo=mongodb&logoColor=white)](https://www.mongodb.com/)

## Features

- **User Management**: User registration, login, and JWT-based authentication
- **Task Management**: Create, read, update, and delete tasks with status tracking and optional icons
- **Authentication**: JWT access and refresh tokens for secure API access
- **Database**: MongoDB integration with BSON serialization
- **Security**: Password hashing with bcrypt, middleware-based authentication

## Tech Stack

- **Language**: Rust (Edition 2024)
- **Framework**: Actix-Web 4.11.0
- **Database**: MongoDB 3.3.0
- **Authentication**: JSON Web Tokens (jsonwebtoken 10.0.0)
- **Password Hashing**: bcrypt 0.17.1
- **Serialization**: Serde with BSON support
- **Date/Time**: Chrono 0.4.42

## Prerequisites

- Rust (latest stable version)
- MongoDB (local or cloud instance)
- Environment variables configured (see .env.example)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust_backend
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Set up environment variables:
   - Copy `.env` and configure your MongoDB URI and JWT secrets
   - Required variables: `MONGO_URI`, `PORT`, `ACCESS_TOKEN_SECRET`, `REFRESH_TOKEN_SECRET`

## Usage

1. Start the server:
   ```bash
   cargo run
   ```

2. The server will run on `http://127.0.0.1:8000` (configurable via PORT environment variable)

## API Endpoints

### Authentication

#### Register User
- **POST** `/users/register`
- **Body**:
  ```json
  {
    "name": "John Doe",
    "email": "john@example.com",
    "password": "password123"
  }
  ```

#### Login User
- **POST** `/users/login`
- **Body**:
  ```json
  {
    "email": "john@example.com",
    "password": "password123"
  }
  ```
- **Response**:
  ```json
  {
    "user": {
      "id": "user_id",
      "name": "John Doe",
      "email": "john@example.com"
    },
    "access_token": "jwt_access_token",
    "refresh_token": "jwt_refresh_token"
  }
  ```

#### Get Users (Protected)
- **GET** `/users/get_user`
- **Headers**: `Authorization: Bearer <access_token>`

### Tasks (All endpoints require authentication)

#### Create Task
- **POST** `/tasks`
- **Headers**: `Authorization: Bearer <access_token>`
- **Body**:
  ```json
  {
    "title": "Task Title",
    "description": "Task description",
    "icon": "📝",
    "status": "Pending",
    "user_id": "user_id"
  }
  ```

#### Get All Tasks for User
- **GET** `/tasks/{user_id}`
- **Headers**: `Authorization: Bearer <access_token>`

#### Get Single Task
- **GET** `/tasks/{user_id}/{task_id}`
- **Headers**: `Authorization: Bearer <access_token>`

#### Update Task
- **PUT** `/tasks/{user_id}/{task_id}`
- **Headers**: `Authorization: Bearer <access_token>`
- **Body**:
  ```json
  {
    "title": "Updated Title",
    "description": "Updated description",
    "icon": "✅",
    "status": "InProgress"
  }
  ```

#### Delete Task
- **DELETE** `/tasks/{user_id}/{task_id}`
- **Headers**: `Authorization: Bearer <access_token>`

## Task Status

Tasks can have the following statuses:
- `Pending`: Initial status when created
- `InProgress`: Task is being worked on
- `Completed`: Task is finished

## Project Structure

```
backend/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── controllers/
│   │   ├── mod.rs
│   │   ├── user_controller.rs  # User-related endpoints
│   │   └── task_controller.rs  # Task-related endpoints
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user_model.rs       # User data structures
│   │   └── task_model.rs       # Task data structures
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── user_routes.rs      # User route configuration
│   │   └── task_routes.rs      # Task route configuration
│   ├── middlewares/
│   │   ├── mod.rs
│   │   └── auth_middleware.rs   # JWT authentication middleware
│   ├── utils/
│   │   ├── mod.rs
│   │   └── jwt.rs              # JWT token utilities
│   ├── db/
│   │   ├── mod.rs
│   │   └── db.rs               # Database connection
│   └── config/                 # Configuration (if any)
├── Cargo.toml                  # Dependencies and project metadata
├── .env                        # Environment variables
└── .gitignore
```

## Environment Variables

Create a `.env` file in the root directory with the following variables:

```
MONGO_URI=mongodb+srv://username:password@cluster.mongodb.net/
PORT=8000
ACCESS_TOKEN_SECRET=your_access_token_secret
REFRESH_TOKEN_SECRET=your_refresh_token_secret
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

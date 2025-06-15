# Rise Up API

## Overview

This project is a backend API implemented in Rust using Actix Web. This project will be used to power Rise Up Voices
---

## Features

- User registration with secure password hashing
- User lookup by email
- User login with password verification
- Custom error handling with proper HTTP status codes
- Clean separation of concerns: handlers, services, repository layers
- Async database interactions with SQLx and PostgreSQL
- JWT or session token support can be added easily (not included by default)

---

## Technologies

- Rust programming language
- Actix Web framework for HTTP server
- SQLx for asynchronous database access
- PostgreSQL as the primary data store
- Bcrypt for password hashing and verification
- ThisError for error definitions and handling
- dotenv for environment configuration

---

## Project Structure

```
src/
├── handlers/           # Actix Web route handlers
├── models/             # Domain models and request/response structs
├── repository/         # Database interaction layer
├── services/           # Business logic and service layer
├── state/              # Application state and dependency injection
├── error/              # Custom error types and conversions
main.rs                 # Application entry point
```

---

## Setup and Running

1. **Configure environment variables**

   Create a `.env` file in the project root with at least:

   ```
   DATABASE_URL=postgres://user:password@localhost/dbname
   ```

2. **Database**

    - Ensure PostgreSQL is running and accessible.
    - Run migrations or create necessary tables (`users` table etc.).

3. **Build and run**

   ```bash
   cargo build
   cargo run
   ```

   The server will start on `127.0.0.1:8080` by default.

---

## API Endpoints

| Method | Endpoint        | Description               | Request Body                 | Response             |
|--------|-----------------|---------------------------|------------------------------|----------------------|
| POST   | `/users`        | Create a new user          | `SubmitUserRequest` JSON      | Created `User` JSON  |
| GET    | `/users/{email}`| Get user by email          | N/A                          | `User` JSON or 404   |
| POST   | `/login`        | Login user                | `LoginRequest` JSON           | `User` JSON or 401   |

---

## Error Handling

Custom errors are defined in the `UserError` enum. These map to proper HTTP status codes:

- `NotFound` → 404
- `InvalidCredentials` → 401
- `DatabaseError` → 500
- `InternalError` → 500

---

## Notes

- Passwords are securely hashed using bcrypt.
- Sensitive data such as hashed passwords should not be exposed in API responses.
- Email addresses in URLs should be URL-encoded.
- Input validation should be implemented for all request payloads.
- Consider adding rate limiting and logging in production.

---

## Contribution

Contributions are welcome! Please fork the repo and open a pull request with improvements.

---

## License

This project is open source and available under the MIT License.

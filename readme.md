# Ecommerce Store

## Overview
Ecommerce Store is a Rust-based web application that provides a backend API for an e-commerce platform. It utilizes MongoDB for data storage and is designed to be deployed on Render.

## Features
- User authentication (JWT-based)
- Product management (CRUD operations)
- Order management
- MongoDB integration
- Deployment configuration for Render

## Tech Stack
- **Backend:** Rust, Actix Web
- **Database:** MongoDB
- **Deployment:** Render

## Installation

### Prerequisites
Ensure you have the following installed:
- [Rust](https://www.rust-lang.org/)
- [MongoDB](https://www.mongodb.com/)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Setup
1. Clone the repository:
   ```sh
   git clone https://github.com/JAESURO/rust-api.git
   cd ecommerce_store
   ```
2. Create a `.env` file based on `.env.example` and set the necessary environment variables.
3. Install dependencies:
   ```sh
   cargo build
   ```
4. Run the application:
   ```sh
   cargo run
   ```

## API Endpoints
| Method | Endpoint          | Description        |
|--------|------------------|--------------------|
| GET    | `/api/products`   | Get all products  |
| POST   | `/api/products`   | Add a product     |
| GET    | `/api/users`      | Get user details  |
| POST   | `/api/auth/login` | Authenticate user |

## Deployment
To deploy on Render, follow these steps:
1. Configure `render.yaml` for service settings.
2. Push the latest code to your GitHub repository.
3. Link the repository to Render and deploy.

## License
This project is licensed under the MIT License.

## Author
Developed by Zhandos Amantaiuly
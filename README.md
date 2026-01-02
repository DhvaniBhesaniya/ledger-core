# Ledger Core

A high-performance, production-ready ledger system built with Rust, featuring role-based access control (RBAC), comprehensive API key management, and real-time transaction processing.

## 🚀 Features

### Core Functionality
- **Account Management** - Create and manage business accounts with multi-currency support
- **Transaction Processing** - Credit/debit transactions with idempotency guarantees
- **Webhook System** - Real-time event notifications for transaction updates
- **API Key Management** - Secure key generation, rotation, and lifecycle management

### Security & Access Control
- **Role-Based Access Control (RBAC)**
  - **Admin Keys** - Full system access, can manage all accounts and API keys
  - **Customer Keys** - Scoped to specific accounts, limited permissions
- **API Key Authentication** - SHA-256 hashed keys with prefix-based identification
- **Rate Limiting** - Configurable per-key rate limits
- **Account Isolation** - Customers can only access their own data

### Developer Experience
- **Comprehensive Logging** - Colored console output with request/response tracking
- **CORS Support** - Configurable cross-origin resource sharing
- **Idempotency** - Prevent duplicate transactions with idempotency keys
- **RESTful API** - Clean, intuitive endpoint design
- **Postman Collection** - Ready-to-use API testing collection

## 📋 Prerequisites

- **Rust** 1.70+ ([Install Rust](https://rustup.rs/))
- **PostgreSQL** 15+ 
- **Docker & Docker Compose** (for containerized setup)
- **Diesel CLI** (for database migrations)

```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres
```

## 🛠️ Installation

### 1. Clone the Repository
```bash
git clone <repository-url>
cd ledger-core
```

### 2. Environment Setup
Create a `.env` file in the project root:

```env
# Database
DATABASE_URL=postgres://postgres:password@localhost:5432/transaction_service

# API Configuration
API_KEY_PREFIX=sk_prod_

# CORS (comma-separated origins)
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://localhost:8080

# Optional
RUST_LOG=info
RUST_ENV=development
```

### 3. Database Setup

#### Option A: Using Docker Compose (Recommended)
```bash
# Start PostgreSQL and Adminer
docker-compose up -d postgres adminer

# Verify database is running
docker-compose ps

# Access Adminer web UI at http://localhost:8081
# Server: postgres | Username: postgres | Password: password
```

#### Option B: Local PostgreSQL
```bash
# Create database
createdb transaction_service

# Or using psql
psql -U postgres -c "CREATE DATABASE transaction_service;"
```

### 4. Run Migrations
```bash
# Apply all migrations
diesel migration run

# Verify schema
diesel migration list
```

### 5. Build and Run
```bash
# Development mode
cargo run

# Production build
cargo build --release
./target/release/ledger-core
```

The server will start on `http://0.0.0.0:8080`

## 🔑 Bootstrap Admin Key

Admin keys are required to manage the system. Create your first admin key:

### 1. Generate UUID and Hash
```bash
# Generate UUID
uuidgen
# Example: ed6477c2-d79f-44f6-986b-42377d6c0be3

# Create full key
# Format: sk_prod_{uuid}
# Example: sk_prod_ed6477c2-d79f-44f6-986b-42377d6c0be3

# Hash the key
echo -n "sk_prod_ed6477c2-d79f-44f6-986b-42377d6c0be3" | sha256sum
# Output: 4d9c8a9fb98de379a361ddd5e9765266b4d88ed8e5d76c40887b6b6fd71fa5df
```

### 2. Insert Admin Key into Database
```bash
# Connect to database
docker-compose exec postgres psql -U postgres -d transaction_service

# Or local psql
psql -U postgres -d transaction_service
```

```sql
INSERT INTO api_keys (
    account_id, key_hash, key_prefix, name,
    is_active, rate_limit_per_minute, role,
    created_at, updated_at
) VALUES (
    NULL,  -- Admin keys don't need an account
    '4d9c8a9fb98de379a361ddd5e9765266b4d88ed8e5d76c40887b6b6fd71fa5df',
    'sk_prod_ed6477c2-d',
    'Bootstrap Admin Key',
    true, 1000, 'admin',
    NOW(), NOW()
);
```

**⚠️ IMPORTANT:** Save your raw API key securely! You cannot retrieve it later.

For detailed instructions, see [ADMIN_BOOTSTRAP.md](./ADMIN_BOOTSTRAP.md)

## 📁 Project Structure

```
ledger-core/
├── src/
│   ├── handlers/          # HTTP request handlers
│   │   ├── account_handlers.rs
│   │   ├── api_key_handlers.rs
│   │   ├── transaction_handlers.rs
│   │   └── webhook_handlers.rs
│   ├── middleware/        # Custom middleware
│   │   ├── api_key_auth.rs      # API key authentication
│   │   ├── authorization.rs     # RBAC authorization helpers
│   │   ├── cors.rs              # CORS configuration
│   │   ├── idempotency.rs       # Idempotency checking
│   │   ├── logging.rs           # Request/response logging
│   │   └── rate_limit.rs        # Rate limiting
│   ├── models/            # Data models
│   │   ├── account.rs
│   │   ├── api_key.rs
│   │   ├── enums.rs             # ApiKeyRole, TransactionStatus, etc.
│   │   ├── transaction.rs
│   │   └── webhook.rs
│   ├── repositories/      # Database access layer
│   │   ├── account_repo.rs
│   │   ├── api_key_repo.rs
│   │   ├── transaction_repo.rs
│   │   └── webhook_repo.rs
│   ├── services/          # Business logic layer
│   │   ├── account_service.rs
│   │   ├── api_key_service.rs
│   │   ├── transaction_service.rs
│   │   └── webhook_service.rs
│   ├── utils/             # Utilities
│   │   ├── app_error.rs         # Error handling
│   │   ├── crypto.rs            # Hashing utilities
│   │   └── db.rs                # Database connection pool
│   ├── routes.rs          # Route definitions
│   ├── schema.rs          # Diesel schema (auto-generated)
│   └── main.rs            # Application entry point
├── migrations/            # Database migrations
├── docker-compose.yml     # Docker services configuration
├── Cargo.toml            # Rust dependencies
├── .env                  # Environment variables (create this)
├── diesel.toml           # Diesel configuration
├── ADMIN_BOOTSTRAP.md    # Admin key setup guide
├── LOGGING.md            # Logging system documentation
├── API_CURL_COMMANDS.md  # API usage examples
└── ledger_core_postman_collection.json  # Postman collection
```

## 🔌 API Endpoints

### Public Endpoints
- `GET /health` - Health check

### Account Management
- `POST /api/accounts` - Create account (public, auto-generates customer key)
- `GET /api/accounts/:id` - Get account details (requires ownership)
- `GET /api/accounts/:id/balance` - Get account balance (requires ownership)
- `GET /api/accounts/:id/keys` - List account API keys (requires ownership)

### API Key Management (Admin Only)
- `POST /api/key_generate` - Generate new API key
- `GET /api/keys_list` - List all API keys
- `PATCH /api/keys/:id` - Update API key

### Transactions
- `POST /api/transactions` - Create transaction (requires customer key)
- `GET /api/transactions/:id` - Get transaction details

### Webhooks
- `POST /api/webhooks` - Register webhook endpoint
- `GET /api/webhooks/:id` - Get webhook details
- `DELETE /api/webhooks/:id` - Delete webhook

## 🧪 Testing

### Using cURL

```bash
# Health check
curl http://localhost:8080/health

# Create account (returns customer API key)
curl -X POST http://localhost:8080/api/accounts \
  -H "Content-Type: application/json" \
  -d '{"business_name": "My Business", "currency": "USD"}'

# Get account (using customer key)
curl http://localhost:8080/api/accounts/1 \
  -H "x-api-key: sk_prod_YOUR_CUSTOMER_KEY"

# List all keys (admin only)
curl http://localhost:8080/api/keys_list \
  -H "x-api-key: sk_prod_YOUR_ADMIN_KEY"
```

See [API_CURL_COMMANDS.md](./API_CURL_COMMANDS.md) for complete examples.

### Using Postman

1. Import `ledger_core_postman_collection.json`
2. Set collection variables:
   - `admin_api_key` - Your admin key
   - `customer_api_key` - Customer key (auto-set on account creation)
   - `base_url` - `http://localhost:8080`
3. Run requests from the collection

## 📊 Logging

The application features comprehensive logging with colored output:

```
2026-01-01T06:25:38Z  INFO ╔════════════════════════════════════════╗
2026-01-01T06:25:38Z  INFO ║   Ledger Core API Server Started       ║
2026-01-01T06:25:38Z  INFO ╠════════════════════════════════════════╣
2026-01-01T06:25:38Z  INFO ║  Address: http://0.0.0.0:8080          ║
2026-01-01T06:25:38Z  INFO ║  Environment: development              ║
2026-01-01T06:25:38Z  INFO ╚════════════════════════════════════════╝

2026-01-01T06:27:32Z  INFO → Incoming request method=GET path=/api/keys_list
2026-01-01T06:27:32Z  INFO ✓ Request completed successfully status=200 duration_ms=26
```

**Log Levels:**
- ✓ Green - Successful requests (200-299)
- ⚠ Yellow - Client errors (400-499)
- ✗ Red - Server errors (500-599)

Control verbosity with `RUST_LOG`:
```bash
RUST_LOG=debug cargo run  # More verbose
RUST_LOG=error cargo run  # Errors only
```

See [LOGGING.md](./LOGGING.md) for detailed documentation.

## 🔐 Security Features

### API Key Security
- Keys are SHA-256 hashed before storage
- Raw keys are only shown once upon generation
- Prefix-based identification (first 20 characters)
- Configurable rate limits per key

### Role-Based Access Control
| Feature | Admin | Customer |
|---------|-------|----------|
| Generate API keys | ✅ | ❌ |
| List all API keys | ✅ | ❌ |
| Update API keys | ✅ | ❌ |
| Access any account | ✅ | ❌ |
| Access own account | ✅ | ✅ |
| Create transactions | ✅ | ✅ |

### Additional Security
- Account isolation for customer keys
- Idempotency key validation
- Rate limiting per API key
- CORS protection
- Input validation and sanitization

## 🐳 Docker Deployment

### Development
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f app

# Stop services
docker-compose down
```

### Production
```bash
# Build production image
docker build -t ledger-core:latest .

# Run with production settings
docker run -d \
  -p 8080:8080 \
  -e DATABASE_URL=postgres://... \
  -e RUST_LOG=info \
  ledger-core:latest
```

## 🛠️ Development

### Running Migrations
```bash
# Create new migration
diesel migration generate migration_name

# Run pending migrations
diesel migration run

# Rollback last migration
diesel migration revert

# Regenerate schema.rs
diesel migration run
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Run tests
cargo test

# Check compilation
cargo check
```

## 📚 Documentation

- [ADMIN_BOOTSTRAP.md](./ADMIN_BOOTSTRAP.md) - Admin key setup guide
- [LOGGING.md](./LOGGING.md) - Logging system documentation
- [API_CURL_COMMANDS.md](./API_CURL_COMMANDS.md) - API usage examples
- [Postman Collection](./ledger_core_postman_collection.json) - API testing collection

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

Built with:
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Diesel](https://diesel.rs/) - ORM and query builder
- [Tokio](https://tokio.rs/) - Async runtime
- [Tracing](https://github.com/tokio-rs/tracing) - Logging framework
- [PostgreSQL](https://www.postgresql.org/) - Database

## 📞 Support

For issues, questions, or contributions, please open an issue on GitHub.

---

**Made with ❤️ using Rust**

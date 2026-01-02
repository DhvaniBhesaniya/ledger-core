# Logging System Documentation

## Overview
The Ledger Core API now features a comprehensive logging system that displays all API calls, errors, and info messages in the terminal with color-coded output.

## Features

### 1. **Colored Console Output**
- ✅ **Green** - Successful requests (200-299)
- ⚠️ **Yellow** - Client errors (400-499)
- ❌ **Red** - Server errors (500-599)

### 2. **Request Logging**
Every incoming request is logged with:
- HTTP Method (GET, POST, PATCH, DELETE, etc.)
- Request path
- Query parameters (if any)
- Response status code
- Request duration in milliseconds

### 3. **Error Tracking**
- Database errors are logged with full error messages
- Internal server errors are logged with context
- Client errors (4xx) are logged as warnings
- Server errors (5xx) are logged as errors

### 4. **Startup Banner**
```
╔════════════════════════════════════════╗
║   Ledger Core API Server Started       ║
╠════════════════════════════════════════╣
║  Address: http://0.0.0.0:8080          ║
║  Environment: development              ║
╚════════════════════════════════════════╝
```

## Example Output

### Successful Request
```
2026-01-01T11:30:00.123Z  INFO → Incoming request method=POST path=/api/accounts
2026-01-01T11:30:00.456Z  INFO ✓ Request completed successfully method=POST path=/api/accounts status=201 duration_ms=333
```

### Client Error (Invalid API Key)
```
2026-01-01T11:30:10.123Z  INFO → Incoming request method=GET path=/api/keys_list
2026-01-01T11:30:10.145Z  WARN ⚠ Client error method=GET path=/api/keys_list status=401 duration_ms=22
2026-01-01T11:30:10.145Z  WARN Client error code=INVALID_API_KEY message="Invalid API key"
```

### Server Error
```
2026-01-01T11:30:20.123Z  INFO → Incoming request method=POST path=/api/transactions
2026-01-01T11:30:20.234Z ERROR Database error occurred error="connection pool exhausted"
2026-01-01T11:30:20.234Z ERROR Server error code=DATABASE_ERROR message="connection pool exhausted"
2026-01-01T11:30:20.234Z ERROR ✗ Server error method=POST path=/api/transactions status=500 duration_ms=111
```

## Configuration

### Log Levels
The logging system uses standard tracing levels:
- `ERROR` - Critical errors
- `WARN` - Warnings and client errors
- `INFO` - General information and successful requests
- `DEBUG` - Detailed debugging (not enabled by default)
- `TRACE` - Very detailed tracing (not enabled by default)

### Environment Variables
You can control log verbosity with the `RUST_LOG` environment variable:

```bash
# Default (INFO level)
cargo run

# Debug level (more verbose)
RUST_LOG=debug cargo run

# Trace level (very verbose)
RUST_LOG=trace cargo run

# Only errors
RUST_LOG=error cargo run

# Module-specific logging
RUST_LOG=ledger_core=debug cargo run
```

### Compact Format
The logging is configured with a compact format that:
- Hides module paths (cleaner output)
- Hides thread IDs
- Shows log levels
- Enables ANSI colors
- Hides file names and line numbers

## Customization

### Adding Custom Logs
You can add custom logs anywhere in your code:

```rust
use tracing::{info, warn, error, debug};

// Info log
tracing::info!("User created account");

// With structured fields
tracing::info!(
    account_id = account.id,
    business_name = %account.business_name,
    "New account created"
);

// Warning
tracing::warn!("Rate limit approaching threshold");

// Error
tracing::error!(error = %e, "Failed to process transaction");

// Debug (only shown with RUST_LOG=debug)
tracing::debug!("Cache hit for key: {}", key);
```

### Modifying Log Format
Edit `src/main.rs` to customize the tracing subscriber:

```rust
tracing_subscriber::fmt()
    .with_target(true)      // Show module paths
    .with_thread_ids(true)  // Show thread IDs
    .with_level(true)
    .with_ansi(true)
    .with_file(true)        // Show file names
    .with_line_number(true) // Show line numbers
    .pretty()               // Use pretty format instead of compact
    .init();
```

## Monitoring

### Watching Logs in Real-Time
```bash
# Run the server
cargo run

# In another terminal, make requests and watch logs
curl http://localhost:8080/health
curl -X POST http://localhost:8080/api/accounts \
  -H "Content-Type: application/json" \
  -d '{"business_name": "Test Corp"}'
```

### Filtering Logs
```bash
# Only show errors
RUST_LOG=error cargo run

# Show specific modules
RUST_LOG=ledger_core::handlers=debug cargo run

# Multiple filters
RUST_LOG=ledger_core::handlers=debug,ledger_core::services=info cargo run
```

## Production Recommendations

### 1. **Structured Logging**
For production, consider using JSON format for easier parsing:
```rust
tracing_subscriber::fmt()
    .json()
    .init();
```

### 2. **Log Aggregation**
Send logs to a centralized logging service:
- ELK Stack (Elasticsearch, Logstash, Kibana)
- Datadog
- CloudWatch (AWS)
- Stackdriver (GCP)

### 3. **Log Rotation**
Use a log rotation tool to prevent disk space issues:
- `logrotate` on Linux
- Built-in rotation with `tracing-appender`

### 4. **Performance**
- Use async logging to avoid blocking
- Set appropriate log levels (INFO or WARN in production)
- Avoid logging sensitive data (API keys, passwords, etc.)

## Troubleshooting

### Logs Not Showing
1. Check `RUST_LOG` environment variable
2. Ensure `tracing_subscriber::fmt::init()` is called in `main()`
3. Verify logging middleware is added to router

### Too Verbose
```bash
# Reduce log level
RUST_LOG=warn cargo run
```

### Missing Colors
- Ensure terminal supports ANSI colors
- Check `.with_ansi(true)` is set in tracing config
- Some CI/CD environments disable colors automatically

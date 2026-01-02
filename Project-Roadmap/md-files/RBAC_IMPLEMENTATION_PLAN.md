# Role-Based API Key Access Control (RBAC)

## Goal
Implement role-based permissions for API keys to differentiate between:
- **Admin Keys**: Full access (create accounts, manage all keys, view all data)
- **Customer Keys**: Limited access (own account only, transactions, webhooks)

## Design Decision: Role vs Permissions

### Option 1: Simple Role Enum
```rust
enum ApiKeyRole {
    Admin,
    Customer,
}
```
**Pros**: Simple, easy to understand
**Cons**: Less flexible for future expansion

### Option 2: Granular Permissions
```rust
struct Permissions {
    can_create_accounts: bool,
    can_manage_all_keys: bool,
    can_view_all_accounts: bool,
    // etc.
}
```
**Pros**: Very flexible
**Cons**: Complex, overkill for current needs

**Recommendation**: Start with **Option 1** (Role enum), easy to migrate to permissions later.

---

## Proposed Changes

### 1. Database Migration
**New file**: `migrations/YYYY-MM-DD-HHMMSS_add_role_to_api_keys/up.sql`
```sql
ALTER TABLE api_keys 
ADD COLUMN role VARCHAR(20) NOT NULL DEFAULT 'customer';

-- Update existing keys to admin (or keep as customer)
-- UPDATE api_keys SET role = 'admin' WHERE id = 1;
```

### 2. Schema & Models

#### [src/schema.rs](file:///home/dhvani/personal_work/ledger-core/src/schema.rs)
Run `diesel migration run` to regenerate.

#### [src/models/enums.rs](file:///home/dhvani/personal_work/ledger-core/src/models/enums.rs)
Add `ApiKeyRole` enum:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ApiKeyRole"]
pub enum ApiKeyRole {
    Admin,
    Customer,
}
```

#### [src/models/api_key.rs](file:///home/dhvani/personal_work/ledger-core/src/models/api_key.rs)
- Add `role: ApiKeyRole` to `ApiKey`, `NewApiKey`, `ApiKeyResponse`
- Add `role: Option<ApiKeyRole>` to `GenerateApiKeyRequest` (defaults to Customer)

### 3. Middleware

#### [src/middleware/api_key_auth.rs](file:///home/dhvani/personal_work/ledger-core/src/middleware/api_key_auth.rs)
Update `ApiKeyAuth` struct:
```rust
pub struct ApiKeyAuth {
    pub account_id: i64,
    pub role: ApiKeyRole,
}
```

### 4. Authorization Middleware

**New file**: [src/middleware/authorization.rs](file:///home/dhvani/personal_work/ledger-core/src/middleware/authorization.rs)
```rust
pub fn require_admin(auth: &ApiKeyAuth) -> Result<(), AppError> {
    if auth.role != ApiKeyRole::Admin {
        return Err(AppError::Forbidden);
    }
    Ok(())
}
```

#### [src/error/app_error.rs](file:///home/dhvani/personal_work/ledger-core/src/error/app_error.rs)
Add `Forbidden` variant (403).

### 5. Route Protection

Update handlers to check roles:
- **Admin-only routes**:
  - `GET /api/keys` (list all)
  - `POST /api/keys` (generate for any account)
  - `PATCH /api/keys/:id` (update any key)
  
- **Customer routes** (can only access own account):
  - `GET /api/accounts/:id` (verify `auth.account_id == id`)
  - `POST /api/transactions` (auto-use `auth.account_id`)

### 6. Services Layer

#### [src/services/api_key_service.rs](file:///home/dhvani/personal_work/ledger-core/src/services/api_key_service.rs)
- `generate_key`: Set default role to `Customer`
- Add admin check for listing all keys

---

## Example Usage

### Admin Key
```bash
# Create admin key (manually in DB or via special endpoint)
curl -X POST http://localhost:8080/api/keys \
  -H "x-api-key: ADMIN_KEY" \
  -d '{"account_id": 1, "role": "admin"}'

# List all keys (admin only)
curl http://localhost:8080/api/keys \
  -H "x-api-key: ADMIN_KEY"
```

### Customer Key
```bash
# Auto-generated on account creation (role: customer)
curl -X POST http://localhost:8080/api/accounts \
  -d '{"business_name": "My Shop"}'

# Customer can only access their own account
curl http://localhost:8080/api/accounts/1 \
  -H "x-api-key: CUSTOMER_KEY"  # ✅ Works if account_id = 1

curl http://localhost:8080/api/accounts/2 \
  -H "x-api-key: CUSTOMER_KEY"  # ❌ 403 Forbidden
```

---

## Verification Plan

1. **Migration**: Run `diesel migration run`
2. **Create admin key**: Manually update one key in DB to `role = 'admin'`
3. **Test admin routes**: Verify admin can list all keys
4. **Test customer isolation**: Verify customer cannot access other accounts
5. **Test forbidden**: Verify 403 responses for unauthorized actions

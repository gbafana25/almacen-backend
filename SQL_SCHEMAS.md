# SQL Schemas

## Users

```sql
users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,

    password_salt BYTEA NOT NULL,

    created_at TIMESTAMP,
    updated_at TIMESTAMP
)
```

## Device
```sql
devices (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),

    name TEXT,

    -- Identity keys (public only)
    identity_public_key BYTEA NOT NULL,

    -- Optional: for onboarding sessions
    last_seen TIMESTAMP,
    created_at TIMESTAMP
)
```

## Vaults
```sql
vaults (
    id UUID PRIMARY KEY,

    name TEXT,

    created_by_device_id UUID REFERENCES devices(id),

    created_at TIMESTAMP
)
```

## Vault Keys
```sql
vault_device_keys (
    id UUID PRIMARY KEY,

    vault_id UUID REFERENCES vaults(id),
    device_id UUID REFERENCES devices(id),

    -- vault key encrypted for this device
    encrypted_vault_key BYTEA NOT NULL,
    nonce BYTEA NOT NULL,

    created_at TIMESTAMP,

    UNIQUE(vault_id, device_id)
)
```

## Vault Items
```sql
vault_items (
    id UUID PRIMARY KEY,

    vault_id UUID REFERENCES vaults(id),

    -- encrypted item data
    ciphertext BYTEA NOT NULL,
    nonce BYTEA NOT NULL,

    -- optional metadata (can also be encrypted)
    created_at TIMESTAMP,
    updated_at TIMESTAMP
)
```

## Device Onboarding Session
```sql
device_onboarding_sessions (
    id UUID PRIMARY KEY,

    requesting_device_id UUID REFERENCES devices(id),

    -- ephemeral public key of new device
    ephemeral_public_key BYTEA NOT NULL,

    -- optional relay payloads (encrypted)
    encrypted_payload BYTEA,

    status TEXT, -- pending | approved | completed

    created_at TIMESTAMP,
    expires_at TIMESTAMP
)
```
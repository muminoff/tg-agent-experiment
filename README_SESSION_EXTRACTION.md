# Telegram Session Data Extraction

This guide explains how to extract authentication data from the Telegram SQLite session file.

## What Data is Stored in the Session File?

The `tg-agent.session` SQLite database contains:

### 1. Authorization Keys
- **auth_key**: 256-byte binary encryption key for API communication
- **WARNING**: This key is SECRET! Anyone with this key can impersonate your account
- Used for encrypting all communication with Telegram servers

### 2. Data Center (DC) Information
- **dc_id**: Integer identifying which Telegram server you're connected to
- Telegram has multiple data centers worldwide (DC1-DC5)
- Examples:
  - DC1: Miami, Florida
  - DC2: Amsterdam, Netherlands
  - DC3: Miami, Florida
  - DC4: Amsterdam, Netherlands
  - DC5: Singapore

### 3. Server Address
- **server_address**: The actual IP or hostname of the Telegram server
- Example: `149.154.167.51:443`

### 4. User ID
- **user_id**: Your unique Telegram user ID (numeric)
- This is permanent and doesn't change

### 5. Session Metadata
- **timestamp**: When the session was created/updated
- **sequence_number**: For message ordering
- **salt**: Cryptographic salt for security

## How to Extract Session Data

### Step 1: Create a Session File

First, run the main program to authenticate and create a session:

```bash
export TG_ID=your_api_id
export TG_HASH=your_api_hash
cargo run
```

This will create `tg-agent.session` after successful authentication.

### Step 2: Run the Extraction Tool

```bash
# Extract from default session file
cargo run --example extract_session_data

# Or specify a different session file
cargo run --example extract_session_data /path/to/custom.session
```

### Expected Output

```
=== Telegram Session Data Extractor ===

Reading session file: tg-agent.session

📋 DATABASE SCHEMA:
============================================================
CREATE TABLE session (
    dc_id INTEGER,
    server_address TEXT,
    auth_key BLOB,
    user_id INTEGER,
    ...
)

📊 AVAILABLE TABLES:
============================================================
  • session

🔍 TABLE: session
============================================================
  Column: dc_id (INTEGER)
  Column: server_address (TEXT)
  Column: auth_key (BLOB)
  Column: user_id (INTEGER)
  ...

  Data:

  Row 1:
    dc_id: 2 (Data Center ID)
    server_address: '149.154.167.51:443' (Telegram Server)
    auth_key: <256 bytes> (Authorization Key - KEEP SECRET!)
    user_id: 123456789 (Your Telegram User ID)
    ...

✅ Extraction complete!
```

## Database Schema Details

The grammers library uses a SQLite database with typically this structure:

```sql
CREATE TABLE session (
    dc_id INTEGER NOT NULL,
    server_address TEXT NOT NULL,
    auth_key BLOB NOT NULL,
    user_id INTEGER,
    ...
);
```

## Security Considerations

### Keep These SECRET:
- **auth_key**: Full access to your account
- **session file**: Contains all authentication data

### Safe to Share:
- **user_id**: Public information (anyone can see it in chats)
- **dc_id**: Not sensitive (just indicates server location)

### Best Practices:
1. Never commit session files to git (add `*.session` to `.gitignore`)
2. Store session files with restricted permissions: `chmod 600 *.session`
3. Never share auth_key with anyone
4. Regenerate session if compromised (logout and login again)

## Alternative: Using Direct SQL

You can also query the database directly with sqlite3:

```bash
# View schema
sqlite3 tg-agent.session ".schema"

# View all tables
sqlite3 tg-agent.session ".tables"

# Query session data
sqlite3 tg-agent.session "SELECT dc_id, server_address, user_id FROM session;"

# Get auth_key length (don't print the key itself!)
sqlite3 tg-agent.session "SELECT length(auth_key) FROM session;"
```

## Using Session Data Programmatically

If you need to access session data in your Rust code:

```rust
use sqlite::{Connection, State};

fn read_session_data(session_file: &str) -> Result<SessionInfo, Box<dyn std::error::Error>> {
    let conn = Connection::open(session_file)?;

    let query = "SELECT dc_id, server_address, user_id FROM session LIMIT 1";
    let mut stmt = conn.prepare(query)?;

    if let State::Row = stmt.next()? {
        let dc_id = stmt.read::<i64, _>(0)?;
        let server_address = stmt.read::<String, _>(1)?;
        let user_id = stmt.read::<i64, _>(2)?;

        Ok(SessionInfo {
            dc_id: dc_id as i32,
            server_address,
            user_id,
        })
    } else {
        Err("No session data found".into())
    }
}

struct SessionInfo {
    dc_id: i32,
    server_address: String,
    user_id: i64,
}
```

## Troubleshooting

### "Session file not found"
Run the main program first to create a session:
```bash
cargo run
```

### "Error opening database"
Check file permissions:
```bash
ls -l *.session
chmod 600 tg-agent.session
```

### "Table not found"
The session file may be corrupted. Delete and recreate:
```bash
rm tg-agent.session
cargo run
```

## References

- [grammers documentation](https://docs.rs/grammers-session/)
- [Telegram MTProto documentation](https://core.telegram.org/mtproto)
- [Telegram API data centers](https://core.telegram.org/api/datacenter)

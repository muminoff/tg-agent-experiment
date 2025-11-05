use sqlite::{Connection, State, Value};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Telegram Session Data Extractor ===\n");

    // Get session file path from command line or use default
    let session_file = env::args()
        .nth(1)
        .unwrap_or_else(|| "tg-agent.session".to_string());

    println!("Reading session file: {}\n", session_file);

    // Check if file exists
    if !std::path::Path::new(&session_file).exists() {
        println!("❌ Session file not found!");
        println!("   Please run the main program first to create a session.");
        return Ok(());
    }

    // Open SQLite database
    let conn = Connection::open(&session_file)?;

    // Display database schema
    println!("📋 DATABASE SCHEMA:");
    println!("{}", "=".repeat(60));

    let query = "SELECT sql FROM sqlite_master WHERE type='table' ORDER BY name";
    let mut stmt = conn.prepare(query)?;

    while let State::Row = stmt.next()? {
        let sql = stmt.read::<String, _>(0)?;
        println!("{}\n", sql);
    }

    // List all tables
    println!("\n📊 AVAILABLE TABLES:");
    println!("{}", "=".repeat(60));

    let query = "SELECT name FROM sqlite_master WHERE type='table'";
    let mut stmt = conn.prepare(query)?;

    let mut tables = Vec::new();
    while let State::Row = stmt.next()? {
        let name = stmt.read::<String, _>(0)?;
        tables.push(name.clone());
        println!("  • {}", name);
    }

    // Extract data from each table
    for table_name in tables {
        println!("\n🔍 TABLE: {}", table_name);
        println!("{}", "=".repeat(60));

        match dump_table_data(&conn, &table_name) {
            Ok(_) => {},
            Err(e) => println!("  ❌ Error reading table: {}", e),
        }
    }

    println!("\n✅ Extraction complete!");

    Ok(())
}

fn dump_table_data(conn: &Connection, table_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!("SELECT * FROM {}", table_name);
    let mut stmt = conn.prepare(&query)?;

    // Get column names using PRAGMA
    let pragma_query = format!("PRAGMA table_info({})", table_name);
    let mut pragma_stmt = conn.prepare(&pragma_query)?;

    let mut column_names = Vec::new();
    while let State::Row = pragma_stmt.next()? {
        let name = pragma_stmt.read::<String, _>(1)?;
        let col_type = pragma_stmt.read::<String, _>(2)?;
        column_names.push((name.clone(), col_type.clone()));
        println!("  Column: {} ({})", name, col_type);
    }

    if column_names.is_empty() {
        println!("  (no columns found)");
        return Ok(());
    }

    println!("\n  Data:");
    let mut row_count = 0;

    while let State::Row = stmt.next()? {
        row_count += 1;
        println!("\n  Row {}:", row_count);

        for (i, (col_name, _)) in column_names.iter().enumerate() {
            // Read value based on its type
            let value_str = match stmt.read::<Value, _>(i) {
                Ok(Value::Integer(v)) => {
                    if col_name == "dc_id" {
                        format!("{} (Data Center ID)", v)
                    } else if col_name == "user_id" {
                        format!("{} (Your Telegram User ID)", v)
                    } else {
                        format!("{}", v)
                    }
                },
                Ok(Value::Float(v)) => format!("{}", v),
                Ok(Value::String(v)) => {
                    if col_name == "server_address" || col_name == "addr" {
                        format!("'{}' (Telegram Server)", v)
                    } else {
                        format!("'{}'", v)
                    }
                },
                Ok(Value::Binary(v)) => {
                    if col_name == "auth_key" || col_name.contains("key") {
                        format!("<{} bytes> (Authorization Key - KEEP SECRET!)", v.len())
                    } else {
                        format!("<{} bytes> (binary data)", v.len())
                    }
                },
                Ok(Value::Null) => "NULL".to_string(),
                Err(e) => format!("(error reading: {})", e),
            };

            println!("    {}: {}", col_name, value_str);
        }
    }

    if row_count == 0 {
        println!("  (no data)");
    }

    Ok(())
}

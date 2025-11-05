use grammers_client::{Client, SignInError};
use grammers_mtsender::SenderPool;
use grammers_session::storages::SqliteSession;
use std::env;
use std::io::{self, BufRead as _, Write as _};
use std::path::Path;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const SESSION_FILE: &str = "tg-agent.session";

fn prompt(message: &str) -> Result<String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    stdin.read_line(&mut line)?;
    Ok(line)
}

async fn async_main() -> Result<()> {
    println!("=== Telegram Agent Experiment ===");
    println!("Minimal Hello World Example\n");

    // Read API credentials from environment variables
    let api_id = env::var("TG_ID")
        .expect("TG_ID environment variable not set")
        .parse::<i32>()
        .expect("TG_ID must be a valid integer");

    let api_hash = env::var("TG_HASH").expect("TG_HASH environment variable not set");

    // Check if session file exists
    let session_exists = Path::new(SESSION_FILE).exists();
    if session_exists {
        println!("Found existing session file: {}", SESSION_FILE);
        println!("Attempting to use saved session...");
    } else {
        println!("No existing session found. Will create new session.");
    }

    println!("Connecting to Telegram...");

    // Create session storage (opens existing or creates new)
    let session = Arc::new(SqliteSession::open(SESSION_FILE)?);

    // Create sender pool and client
    let pool = SenderPool::new(Arc::clone(&session), api_id);
    let client = Client::new(&pool);
    let SenderPool { runner, .. } = pool;
    let _ = tokio::spawn(runner.run());

    println!("Connected successfully!");

    // Check if we're already signed in
    if !client.is_authorized().await? {
        println!("\nSession found but not authorized. Starting sign-in process...");

        // Get phone number from user
        let phone = prompt("Enter your phone number (international format): ")?;
        let token = client.request_login_code(&phone, &api_hash).await?;

        // Get verification code from user
        let code = prompt("Enter the code you received: ")?;

        // Sign in
        let signed_in = client.sign_in(&token, &code).await;

        match signed_in {
            Err(SignInError::PasswordRequired(password_token)) => {
                // 2FA is enabled, ask for password
                let hint = password_token.hint().unwrap_or("None");
                let prompt_message = format!("Enter your password (hint: {}): ", hint);
                let password = prompt(&prompt_message)?;

                client
                    .check_password(password_token, password.trim())
                    .await?;
                println!("Signed in successfully!");
            }
            Ok(_) => {
                println!("Signed in successfully!");
            }
            Err(e) => return Err(e.into()),
        }

        println!("Session saved to {}", SESSION_FILE);
    } else {
        println!("Successfully authenticated using existing session!");
    }

    // Get current user info
    let me = client.get_me().await?;
    println!(
        "\n✓ Hello, {}!",
        me.first_name().unwrap_or("Unknown")
    );
    println!("✓ User ID: {}", me.raw.id());

    println!("\n=== Connection successful! ===");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    async_main().await
}

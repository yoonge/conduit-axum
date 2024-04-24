use anyhow::{anyhow, Context};
use tokio::task;

use argon2::{
    password_hash::{self, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier
};

pub async fn hash(password: String) -> anyhow::Result<String> {
    task::spawn_blocking(move || {
        let secret = std::env::var("HASH_SALT").expect("HASH_SALT must be set");
        let salt_str = SaltString::encode_b64(&secret.as_bytes()).unwrap();

        anyhow::Ok(
            Argon2::default()
                .hash_password(password.as_bytes(), &salt_str)
                .map_err(|e| anyhow!(e).context("Failed to hash password."))?
                .to_string(),
        )
    })
    .await?
    .context("Panic in password hash().")
}

pub async fn _verify(password: String, hash: String) -> anyhow::Result<bool> {
    task::spawn_blocking(move || {
        let hash = PasswordHash::new(&hash)
            .map_err(|e| anyhow!(e).context("BUG: Password hash is invalid."))?;

        let res = Argon2::default().verify_password(password.as_bytes(), &hash);

        match res {
            Ok(_) => anyhow::Ok(true),
            Err(password_hash::Error::Password) => anyhow::Ok(false),
            Err(e) => Err(anyhow!(e).context("Failed to verify password.")),
        }
    })
    .await?
    .context("Panic in password verify().")
}

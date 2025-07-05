use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let issuer_url_string = env::var("ISSUER_URL").map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("ISSUER_URL environment variable not set: {}", e),
        )
    })?;

    match rs_oidc_provider_metadata::get_provider_json_from_url_string(issuer_url_string).await {
        Ok(json) => {
            io::stdout().write_all(json.as_bytes())?;
            Ok(())
        }
        Err(e) => {
            io::stderr().write_all(format!("Error: {e}",).as_bytes())?;
            Err(io::Error::other("Failed to fetch provider metadata"))
        }
    }
}

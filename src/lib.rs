use std::io;

use openidconnect::IssuerUrl;
use openidconnect::core::CoreProviderMetadata;

pub fn str2issuer_url(s: String) -> Result<IssuerUrl, io::Error> {
    IssuerUrl::new(s).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

pub async fn fetch_provider_metadata(issuer_url: IssuerUrl) -> Result<String, io::Error> {
    let cbld: reqwest::ClientBuilder =
        reqwest::ClientBuilder::new().redirect(reqwest::redirect::Policy::none());
    let client = cbld.build().map_err(|e| io::Error::other(e.to_string()))?;
    let metadata = CoreProviderMetadata::discover_async(issuer_url, &client)
        .await
        .map_err(|e| io::Error::other(e.to_string()))?;
    serde_json::to_string_pretty(&metadata).map_err(|e| io::Error::other(e.to_string()))
}

pub async fn get_provider_json_from_url_string(
    issuer_url_string: String,
) -> Result<String, io::Error> {
    let issuer_url = str2issuer_url(issuer_url_string)?;
    let provider_json = fetch_provider_metadata(issuer_url).await?;
    Ok(provider_json)
}

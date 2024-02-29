use reqwest::{
    header::{HeaderMap, HeaderName},
    Client, StatusCode,
};

// Scalable Auth
mod access_token;
use access_token::AccessToken;
// End Scalable Auth

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quota_project = env!("QUOTA_PROJECT_ID");
    let uri = "default";
    let scopes = "https://www.googleapis.com/auth/cloud-platform";

    // Scalable Auth
    let token = AccessToken::from_uri(uri, scopes).await?;
    // End Scalable Auth
    println!("{:#?}", token);

    let client = Client::new();

    let mut headers = HeaderMap::new();
    // Scalable Auth
    let (k, v) = token.raw_authorization_header().await?;
    // End Scalable Auth
    headers.insert(HeaderName::try_from(k)?, v.parse()?);

    let k = "x-goog-user-project";
    headers.insert(HeaderName::try_from(k)?, quota_project.parse()?);

    let url = "https://translation.googleapis.com/language/translate/v2/?q=Helloworld&target=sv";
    let request = client.get(url).headers(headers).build()?;
    let response = client.execute(request).await?;

    assert!(response.status() == StatusCode::OK);
    println!("{}", response.text().await?);

    Ok(())
}

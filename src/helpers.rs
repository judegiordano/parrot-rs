use lambda_web::actix_web::HttpRequest;

use crate::env;

pub async fn authenticate(req: HttpRequest) -> anyhow::Result<()> {
    let config = env::Config::new()?;
    let token = match req.headers().get("Authorization") {
        Some(value) => value.to_str(),
        None => anyhow::bail!("missing authentication header"),
    }?;
    if token != config.authentication_token {
        anyhow::bail!("invalid authentication token")
    }
    Ok(())
}

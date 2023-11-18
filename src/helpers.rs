use lambda_web::actix_web::HttpRequest;

use crate::env;

pub async fn authenticate(req: HttpRequest) -> anyhow::Result<()> {
    let config = env::Config::new()?;
    let bearer_token = match req.headers().get("Authorization") {
        Some(value) => value.to_str(),
        None => anyhow::bail!("missing authentication header"),
    }?;
    let parts = bearer_token.split(' ').collect::<Vec<_>>();
    let token = match parts.get(1) {
        Some(token) => *token,
        None => anyhow::bail!("missing authentication token"),
    };
    if token != config.authentication_token {
        anyhow::bail!("invalid authentication token")
    }
    Ok(())
}

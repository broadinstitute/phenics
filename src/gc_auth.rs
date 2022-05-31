use google_cloud_auth::{Config, create_token_source};
use google_cloud_auth::token_source::TokenSource;
use google_cloud_auth::token::Token;
use crate::error::Error;

const GCS_FULL_CONTROL_SCOPE: &str = "https://www.googleapis.com/auth/devstorage.full_control";

pub(crate) struct GCAuth {
    token_source: Box<dyn TokenSource>
}

impl GCAuth {
    pub(crate) async fn new() -> Result<GCAuth, Error> {
        let audience: Option<&str> = None;
        let scopes = Some(&[ GCS_FULL_CONTROL_SCOPE ][..]);
        let config = Config { audience, scopes };
        let token_source = create_token_source(config).await?;
        Ok(GCAuth { token_source })
    }
    pub(crate) async fn get_token(&self) -> Result<Token, Error> {
        Ok(self.token_source.token().await?)
    }
}
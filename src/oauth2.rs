use std::future::Future;
use tokio::net::TcpListener;
use url::Url;

pub trait Oauth2Trait<'a> {
    fn auth_request_url(&self, scope: &'a str) -> Result<Url, url::ParseError>;
    fn chech_state(&self, state: &str) -> bool;
    fn get_token(
        &self,
        code: &str,
    ) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> + Send;
    fn run(
        &self,
        listener: TcpListener,
    ) -> impl Future<Output = Result<(String, String), std::io::Error>> + Send;
}

use std::{collections::HashMap, path::PathBuf, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::TcpListener,
    time,
};
use url::Url;

use crate::query_find;

pub struct Oauth2<'a> {
    target: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    pub save_path: PathBuf,
    duration: u64,
    state: &'a str,
    pub redirect_url: &'a str,
}

impl<'a> Oauth2<'a> {
    pub fn new(
        target: &'a str,
        client_id: &'a str,
        client_secret: &'a str,
        duration: u64,
        state: &'a str,
        save_path: PathBuf,
        redirect_url: &'a str,
    ) -> Oauth2<'a> {
        Oauth2 {
            target,
            client_id,
            client_secret,
            duration,
            state,
            save_path,
            redirect_url,
        }
    }

    pub fn chech_state(&self, state: &str) -> bool {
        self.state == state
    }

    pub fn auth_request_url(&self, scope: &'a str) -> Result<Url, url::ParseError> {
        url::Url::parse(&format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            self.target, self.client_id, self.redirect_url, scope, self.state
        ))
    }

    pub async fn get_token(&self, code: &str) -> Result<reqwest::Response, reqwest::Error> {
        let reqwest_client = reqwest::Client::new();

        let mut params = HashMap::new();
        params.insert("client_id", self.client_id);
        params.insert("client_secret", self.client_secret);
        params.insert("code", code);
        params.insert("grant_type", "authorization_code");
        params.insert("redirect_uri", self.redirect_url);

        reqwest_client
            .post("https://id.twitch.tv/oauth2/token")
            .form(&params)
            .send()
            .await
    }

    pub async fn run(&self, listener: TcpListener) -> Result<(String, String), std::io::Error> {
        tokio::select! {
                stream = listener.accept()=>{
                    match stream {
                        Ok((mut stream, _)) =>  {
                            let code;
                            let rev_state;
                            {
                                let mut reader = tokio::io::BufReader::new(&mut stream);
                                let mut request_line = String::new();
                                reader.read_line(&mut request_line).await?;
                                println!("request line: {:?}", &request_line);
                                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                                let url = url::Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                                code = query_find(&url, "code");
                                rev_state = query_find(&url, "state");
                            }

                            let message = "Go back to your terminal :)";
                            let response = format!(
                                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                                message.len(),
                                message
                            );
                            stream.write_all(response.as_bytes()).await?;

                            Ok((code,rev_state))

                        },
                    Err(e)=>{Err(e)}
                    }
                }
                _= time::sleep(Duration::from_secs(self.duration)) =>{
                    println!("timeout");
                    Err(std::io::ErrorKind::TimedOut.into())
            }

        }
    }
}

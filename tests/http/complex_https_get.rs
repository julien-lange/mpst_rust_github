use hyper::{Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use std::fs;

#[tokio::main]
async fn aux() -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    match fs::read_to_string("imgur.env") {
        Ok(contents) => {
            let lines: Vec<&str> = contents.split('\n').collect();

            let s = RandomState::new();
            let mut ids: HashMap<&str, &str> = HashMap::with_hasher(s);

            for line in lines {
                let temp: Vec<&str> = line.split('=').collect();
                ids.insert(temp[0], temp[1]);
            }

            let req = Request::builder()
                .method(Method::GET)
                .uri(ids["CREDITS_URL"])
                .header("content-type", ids["CONTENT_TYPE"])
                .header(
                    "Authorization",
                    format!("{} {}", ids["TOKEN_TYPE"], ids["ACCESS_TOKEN"]),
                )
                .header("User-Agent", ids["USER_AGENT"])
                .header("Accept", ids["ACCEPT"])
                .header("Connection", ids["CONNECTION"])
                .body(Body::default())?;

            let https = HttpsConnector::new();

            let client = Client::builder().build::<_, Body>(https);

            // Await the response...
            // let mut resp = client.request(req).await?;

            // And now...
            // while let Some(chunk) = resp.body_mut().data().await {
            //     stdout().write_all(&chunk?).await?;
            // }

            // Ok(resp)

            Ok(client.request(req).await?)
        }
        Err(_) => Ok(Response::default()),
    }
}

pub fn main() {
    assert!(aux().is_ok());
}

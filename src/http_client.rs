use reqwest::{
    Response,
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, HeaderMap, HeaderName, HeaderValue, REFERER,
    },
};

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        let mut headers = HeaderMap::new();

        headers.insert(
            ACCEPT,
            HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            ),
        );
        headers.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_static("pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7"),
        );
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br, zstd"),
        );
        headers.insert(
            REFERER,
            HeaderValue::from_static("https://www.pichau.com.br"),
        );
        headers.insert(
            HeaderName::from_static("alt-used"),
            HeaderValue::from_static("www.pichau.com.br"),
        );

        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:153.0) Gecko/20100101 Firefox/153.0")
            .default_headers(headers)
            .build()?;

        return Ok(Self { client });
    }

    // faz uma request e retorna ela ou o erro
    pub async fn new_request(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let response: Result<Response, reqwest::Error> = self.client.get(url).send().await;

        return response;
    }
}

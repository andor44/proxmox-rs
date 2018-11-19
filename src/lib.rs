#[macro_use] extern crate log;

extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate reqwest;
extern crate url;

use std::default::Default;
use reqwest::Url;

//pub mod resources;

pub struct Client {
    client: reqwest::Client,
    api_url: Url,
    auth_token: String,
    csrf_token: String,
}

#[derive(Default)]
pub struct ClientBuilder {
    api_url: Option<Url>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Debug)]
pub enum ClientBuildingError {
    MissingParameter(&'static str),
    ReqwestError(reqwest::Error),
}

// Proxmox wraps all JSON responses in a `data` key. This is a polyfill for that.
#[derive(Deserialize)]
struct ProxmoxResponse<T> {
    data: T,
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    #[serde(rename = "CSRFPreventionToken")]
    csrf_token: String,
    #[serde(rename = "ticket")]
    auth_token: String,
    username: String,
}

impl ClientBuilder {
    pub fn new() -> Self {
        ClientBuilder {
            .. Default::default()
        }
    }
    pub fn api_url<T: Into<Url>>(self, api_url: T) -> Self {
        ClientBuilder {
            api_url: Some(api_url.into()),
            .. self
        }
    }
    pub fn username(self, username: String) -> Self {
        ClientBuilder {
            username: Some(username),
            .. self
        }
    }
    pub fn password(self, password: String) -> Self {
        ClientBuilder {
            password: Some(password),
            .. self
        }
    }
    pub fn build(self) -> Result<Client, ClientBuildingError> {
        let api_url = self.api_url.ok_or(ClientBuildingError::MissingParameter("API URL"))?;
        let username = self.username.ok_or(ClientBuildingError::MissingParameter("username"))?;
        let password = self.password.ok_or(ClientBuildingError::MissingParameter("password"))?;
        let builder = reqwest::ClientBuilder::new();
        // TODO: remove acceptance of invalid certs
        let client = builder
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(ClientBuildingError::ReqwestError)?;
        let form = json!({
            "username": username,
            "password": password,
        });
        trace!("attempting to obtain auth ticket");
        let token_request = client
            .post(api_url.join("access/ticket").expect("Invalid URL?"))
            .form(&form);
        let response: ProxmoxResponse<TokenResponse> = token_request
            .send()
            .map_err(ClientBuildingError::ReqwestError)?
            .json()
            .map_err(ClientBuildingError::ReqwestError)?;
        Ok(Client {
            client,
            api_url,
            auth_token: response.data.auth_token,
            csrf_token: response.data.csrf_token,
        })
    }
}

impl Client {
    pub fn get<T>(&self, path: &str) -> reqwest::Result<T>
        where T: serde::de::DeserializeOwned,
    {
        self.request(reqwest::Method::GET, path, Option::None::<&()>, Option::None::<&()>)
    }

    pub fn get_with_query<T, Q>(&self, path: &str, query: &Q) -> reqwest::Result<T>
        where T: serde::de::DeserializeOwned,
              Q: serde::Serialize,
    {
        self.request(reqwest::Method::GET, path, Option::None::<&()>, Some(query))
    }

    pub fn post<T, F>(&self, path: &str, body: &F) -> reqwest::Result<T>
        where T: serde::de::DeserializeOwned,
              F: serde::Serialize,
    {
        self.request(reqwest::Method::POST, path, Some(body), Option::None::<&()>)
    }

    pub fn request<T, Q, F>(&self,
                            method: reqwest::Method,
                            path: &str,
                            form: Option<&F>,
                            query: Option<&Q>) -> reqwest::Result<T>
        where Q: serde::Serialize,
              F: serde::Serialize,
              T: serde::de::DeserializeOwned,
    {
        const WRITING_METHODS: [reqwest::Method; 3] = [
            reqwest::Method::POST,
            reqwest::Method::PUT,
            reqwest::Method::DELETE,
        ];
        let needs_csrf_token = WRITING_METHODS.contains(&method);
        // TODO: https://github.com/seanmonstar/reqwest/issues/261
        let url = self.api_url.join(path).expect("Invalid URL");
        let mut request = self.client.request(method, url);
        request = self.authorize(request);
        if needs_csrf_token {
            request = self.protect_csrf(request);
        }
        if let Some(query) = query {
            request = request.query(query);
        }
        if let Some(form) = form {
            request = request.form(form);
        }
        request
            .send()
            .and_then(|mut response| response.json::<ProxmoxResponse<T>>())
            .map(|wrapper| wrapper.data)
    }

    fn authorize(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request.header(reqwest::header::COOKIE, format!("PVEAuthCookie={}", self.auth_token))
    }

    fn protect_csrf(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request.header("CSRFPreventionToken", self.csrf_token.clone())
    }
}

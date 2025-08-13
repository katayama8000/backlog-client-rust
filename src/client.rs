use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum BacklogError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },
    #[error("Authentication failed")]
    Auth,
    #[error("Backlog error")]
    Space(#[from] crate::types::error::ErrorResponse),
}

pub type BacklogResult<T> = std::result::Result<T, BacklogError>;

/// Backlog client with API key authentication
#[derive(Clone)]
pub struct BacklogClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl BacklogClient {
    /// Create a new Backlog client with API key authentication
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of your Backlog instance (e.g., "https://yourspace.backlog.com" or "https://yourspace.backlogtool.com")
    /// * `api_key` - Your Backlog API key
    ///
    /// # Example
    ///
    /// ```
    /// use backlog_client::BacklogClient;
    ///
    /// let client = BacklogClient::new("https://yourspace.backlog.com", "your_api_key");
    /// ```
    pub fn new<S: Into<String>>(base_url: S, api_key: S) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            client: Client::new(),
        }
    }

    /// Create a new Backlog client with custom reqwest client
    pub fn with_client<S: Into<String>>(base_url: S, api_key: S, client: Client) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            client,
        }
    }

    /// Build URL with API key parameter
    fn build_url(&self, endpoint: &str) -> BacklogResult<Url> {
        let base_url = if self.base_url.ends_with('/') {
            &self.base_url[..self.base_url.len() - 1]
        } else {
            &self.base_url
        };

        let mut url = Url::parse(&format!("{base_url}{endpoint}"))?;
        url.query_pairs_mut().append_pair("apiKey", &self.api_key);
        Ok(url)
    }

    /// Make a GET request to the Backlog API
    pub async fn get(&self, endpoint: &str) -> BacklogResult<Response> {
        let url = self.build_url(endpoint)?;
        let response = self.client.get(url).send().await?;
        self.handle_response(response).await
    }

    /// Make a POST request to the Backlog API
    pub async fn post<T: Serialize>(&self, endpoint: &str, body: &T) -> BacklogResult<Response> {
        let url = self.build_url(endpoint)?;
        let response = self.client.post(url).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Make a PUT request to the Backlog API
    pub async fn put<T: Serialize>(&self, endpoint: &str, body: &T) -> BacklogResult<Response> {
        let url = self.build_url(endpoint)?;
        let response = self.client.put(url).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Make a PATCH request to the Backlog API
    pub async fn patch<T: Serialize>(&self, endpoint: &str, body: &T) -> BacklogResult<Response> {
        let url = self.build_url(endpoint)?;
        let response = self.client.patch(url).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Make a DELETE request to the Backlog API
    pub async fn delete(&self, endpoint: &str) -> BacklogResult<Response> {
        let url = self.build_url(endpoint)?;
        let response = self.client.delete(url).send().await?;
        self.handle_response(response).await
    }

    /// Handle API response and check for errors
    async fn handle_response(&self, response: Response) -> BacklogResult<Response> {
        let status = response.status();
        if status.is_success() {
            Ok(response)
        } else if status == 401 {
            Err(BacklogError::Auth)
        } else {
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(BacklogError::Api {
                status: status.as_u16(),
                message: error_message,
            })
        }
    }

    /// Get JSON response from endpoint
    pub async fn get_json<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> BacklogResult<T> {
        let response = self.get(endpoint).await?;
        let json = response.json::<T>().await?;
        Ok(json)
    }

    /// Post JSON and get JSON response
    pub async fn post_json<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> BacklogResult<R> {
        let response = self.post(endpoint, body).await?;
        let json = response.json::<R>().await?;
        Ok(json)
    }
}

impl BacklogClient {
    // Space API methods

    /// Get space information
    pub async fn get_space(&self) -> BacklogResult<crate::types::space::Space> {
        self.get_json("/api/v2/space").await
    }
}

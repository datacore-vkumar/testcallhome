use std::time::Duration;
use reqwest::{Client, Url};
use crate::api_models::{NodesApi, PoolsApi, ReplicasApi, VolumesApi};
use url::{Url as OtherUrl};
use snafu::Snafu;
const APIVERSION: &str = "v0";

#[derive(Debug, Snafu)]
#[allow(clippy::enum_variant_names)]
pub enum ReqwestClientError {
    #[snafu(display(
    "Http Error : {}",name))]
    /// Error generated when the loop stops processing
    HttpError {
        name: String,
    },
    #[snafu(display("Json Parse Error : {}", source))]
    SerdeError {
        source: serde_json::Error,
    },
    #[snafu(display("Reqwest client error: {}", source))]
    ReqwestError {
        source: reqwest::Error,
    },
    #[snafu(display("Url Parse Error: {}", source))]
    ParseError {
        source: url::ParseError,
    },
    Noun {},
}
/*
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub(crate) enum ReqwestClientError {
    ResourceError(Box<dyn std::error::Error>),
    HttpError(String),
    SerdeError(serde_json::Error),
    ReqwestError(reqwest::Error),
    ParseError(url::ParseError)
}
impl From<Box<dyn std::error::Error>> for ReqwestClientError {
    fn from(e: Box<dyn std::error::Error>) -> ReqwestClientError {
        ReqwestClientError::ResourceError(e)
    }
}
*/

impl From<String> for ReqwestClientError {
    fn from(name: String) -> Self {
        Self::HttpError{name}
    }
}
impl From<serde_json::Error> for ReqwestClientError {
    fn from(source: serde_json::Error) -> Self {
        Self::SerdeError{source}
    }
}

impl From<reqwest::Error> for ReqwestClientError {
    fn from(source: reqwest::Error) -> Self {
        Self::ReqwestError{source}
    }
}

impl From<url::ParseError> for ReqwestClientError {
    fn from(source: url::ParseError) -> Self {
        Self::ParseError {source}
    }
}
impl ReqwestClientError {
    /// Returns K8sResourceError from provided message
    pub fn invalid_http_response_error(name: String) -> Self {
        Self::HttpError{name}
    }
}
#[derive(Clone)]
pub struct ReqwestClient {
    client: Client,
    base_url: Url
}

impl ReqwestClient {
    pub(crate) fn new(url: &str) -> Result<Self, ReqwestClientError>
    {
        let client = reqwest::Client::builder().timeout(Duration::from_secs(60)).build()?;
        let base_url = OtherUrl::parse(url)?;
        Ok(Self {
            client,
            base_url
        })
    }

    pub async fn get_pools(&self) -> Result<Vec<PoolsApi>, ReqwestClientError> {
        let path = format!("/{}/pools",APIVERSION);
        let url = self.base_url.join(&path)?;
        let response = self.client.get(url)
            .send()
            .await?;
        match response.status().is_success(){
            true => {
                let pools = response.json::<Vec<PoolsApi>>().await?;
                Ok(pools)
            }
            false => Err(ReqwestClientError::invalid_http_response_error(response.error_for_status().err().unwrap().to_string()))
        }
    }

    pub async fn get_nodes(&self) -> Result<Vec<NodesApi>, ReqwestClientError> {
        let path = format!("/{}/nodes",APIVERSION);
        let url = self.base_url.join(&path)?;
        let response = self.client.get(url)
            .send()
            .await?;
        match response.status().is_success(){
            true => {
                let nodes = response.json::<Vec<NodesApi>>().await?;
                Ok(nodes)
            }
            false => Err(ReqwestClientError::invalid_http_response_error(response.error_for_status().err().unwrap().to_string()))
        }
    }

    pub async fn get_volumes(&self, max_entries: u32) -> Result<VolumesApi, ReqwestClientError> {
        let path = format!("/{}/volumes",APIVERSION);
        let url = self.base_url.join(&path)?;
        let response = self.client.get(url)
            .query(&[("max_entries", max_entries)])
            .send()
            .await?;
        match response.status().is_success(){
            true => {
                let volumes = response.json::<VolumesApi>().await?;
                Ok(volumes)
            }
            false => Err(ReqwestClientError::invalid_http_response_error(response.error_for_status().err().unwrap().to_string()))
        }
    }

    pub async fn get_replicas(&self) -> Result<Vec<ReplicasApi>, ReqwestClientError> {
        let path = format!("/{}/replicas",APIVERSION);
        let url = self.base_url.join(&path)?;
        let response = self.client.get(url)
            .send()
            .await?;
        match response.status().is_success() {
            true => {
                let replicas = response.json::<Vec<ReplicasApi>>().await?;
                Ok(replicas)
            }
            false => Err(ReqwestClientError::invalid_http_response_error(response.error_for_status().err().unwrap().to_string()))
        }
    }
}


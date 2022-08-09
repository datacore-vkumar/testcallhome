use std::time::Duration;
use reqwest::{Client, Response, StatusCode, Url};
use reqwest::ClientBuilder as ReqwestClientBuilder;
use reqwest::RequestBuilder as ReqwestRequestBuilder;
use reqwest::Response as ReqwestResponse;
use crate::api_models::{NodesApi, PoolsApi, ReplicasApi, VolumesApi};
use url::{Url as OtherUrl, ParseError};


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

impl From<String> for ReqwestClientError {
    fn from(e: String) -> ReqwestClientError {
        ReqwestClientError::HttpError(e)
    }
}
impl From<serde_json::Error> for ReqwestClientError {
    fn from(e: serde_json::Error) -> ReqwestClientError {
        ReqwestClientError::SerdeError(e)
    }
}

impl From<reqwest::Error> for ReqwestClientError {
    fn from(e: reqwest::Error) -> ReqwestClientError {
        ReqwestClientError::ReqwestError(e)
    }
}

impl From<url::ParseError> for ReqwestClientError {
    fn from(e: url::ParseError) -> ReqwestClientError {
        ReqwestClientError::ParseError(e)
    }
}
impl ReqwestClientError {
    /// Returns K8sResourceError from provided message
    pub fn invalid_http_response_error(err: String) -> Self {
        Self::HttpError(err)
    }
}

pub(crate) struct ReqwestClient {
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
        let url = self.base_url.join("/pools")?;
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
        let url = self.base_url.join("/nodes")?;
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
        let url = self.base_url.join("/volumes")?;
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
        let url = self.base_url.join("/replicas")?;
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
/*
impl ReqwestClient {
    pub fn new(url: &str) -> Result<Self, reqwest::Error>
    {
        let client = reqwest::Client::builder().timeout(Duration::from_secs(60)).build()?;
        let base_url = Url::parse(url)?;
        Ok(Self {
            client,
            base_url
        })
    }

    pub async fn get_pools(&self) -> Option<Vec<PoolsApi>> {
        let url = self.base_url.join("/vol3").unwrap();
        let response = self.client.get(url)
            .send()
            .await;
        let pools_object = match response {
            Ok(response) => {
                match response.status().is_success()
                {
                    true => {
                        let pools_object = response.json::<Vec<PoolsApi>>().await;
                        match pools_object {
                            Ok(pools_object) =>  Some(pools_object),
                            Err(err) => {
                                println!("Faileed to parse");
                                None
                            }
                        }
                    },
                    false => {
                        println!("{}", response.error_for_status().err().unwrap());
                        None
                    }
                }
            },
            Err(err) => {
                println!("{:?}", err);
                None
            }
        };
        pools_object
    }

    pub async fn get_nodes(&self) -> Option<Vec<NodesApi>> {
        let url = self.base_url.join("/vol3").unwrap();
        let response = self.client.get(url)
            .send()
            .await;
        let nodes_object = match response {
            Ok(response) => {
                match response.status().is_success()
                {
                    true => {
                        let nodes_object = response.json::<Vec<NodesApi>>().await;
                        match nodes_object {
                            Ok(nodes_object) =>  Some(nodes_object),
                            Err(err) => {
                                println!("Faileed to parse");
                                None
                            }
                        }
                    },
                    false => {
                        println!("{}", response.error_for_status().err().unwrap());
                        None
                    }
                }
            },
            Err(err) => {
                println!("{:?}", err);
                None
            }
        };
        nodes_object
    }

    pub async fn get_volumes(&self) -> Option<VolumesApi> {
        let url = self.base_url.join("/vol3").unwrap();
        let response = self.client.get(url)
            .send()
            .await;
        let volumes_object = match response {
            Ok(response) => {
                match response.status().is_success()
                {
                    true => {
                        let volumes_object = response.json::<VolumesApi>().await;
                        match volumes_object {
                            Ok(volumes_object) =>  Some(volumes_object),
                            Err(err) => {
                                println!("Faileed to parse");
                                None
                            }
                        }
                    },
                    false => {
                        println!("{}", response.error_for_status().err().unwrap());
                        None
                    }
                }
            },
            Err(err) => {
                println!("{:?}", err);
                None
            }
        };
        volumes_object
    }

    pub async fn get_replicas(&self) -> Option<Vec<ReplicasApi>> {
        let url = self.base_url.join("/vol3").unwrap();
        let response = self.client.get(url)
            .send()
            .await;
        let replicas_object = match response {
            Ok(response) => {
                match response.status().is_success()
                {
                    true => {
                        let replicas_object = response.json::<Vec<ReplicasApi>>().await;
                        match replicas_object {
                            Ok(replicas_object) =>  Some(replicas_object),
                            Err(err) => {
                                println!("Faileed to parse");
                                None
                            }
                        }
                    },
                    false => {
                        println!("{}", response.error_for_status().err().unwrap());
                        None
                    }
                }
            },
            Err(err) => {
                println!("{:?}", err);
                None
            }
        };
        replicas_object
    }
}

 */

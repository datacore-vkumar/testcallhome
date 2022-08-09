mod api_models;
mod report_models;
mod client;
mod k8s_client;
use log::{debug, error, log_enabled, info, Level};

use std::error::Error;
use crate::report_models::{Pools, Replicas, Report, Volumes};
use std::time::Duration as OtherDuration;

use futures::{SinkExt, StreamExt, TryStreamExt};
use tokio;
use sha256::digest;
use crate::client::{ReqwestClient, ReqwestClientError};
use crate::k8s_client::{ClientSet, K8sResourceError};

const PRODUCT: &str = "Bolt";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let k =generate_report().await.unwrap();


    Ok(())
}

pub async fn generate_report() -> Result<(), Box<dyn std::error::Error>>
{
    let k8s_client = ClientSet::new(None).await.unwrap();
    let mut report = Report::new();
    report.product_name = Some(PRODUCT.to_string());
    let k8s_node_count = k8s_client.get_nodes().await;
    match k8s_node_count {
        Ok(k8s_node_count) => report.k8s_node_count = Some(k8s_node_count as u64),
        Err(err) => {
            error!("{:?}",err);
        }
    };
    let k8s_cluster_id = k8s_client.get_cluster_id().await;
    match k8s_cluster_id {
        Ok(k8s_cluster_id) => report.k8s_cluster_id = Some(digest(k8s_cluster_id)),
        Err(err) => {
            error!("{:?}",err);
        }
    };
    let reqwest_client = ReqwestClient::new("https://68754317-a104-4536-8d54-53130873a100.mock.pstmn.io").unwrap();
    let nodes = reqwest_client.get_nodes().await;
    match nodes {
        Ok(nodes) => report.storage_node_count = Some(nodes.len() as u64),
        Err(err) => {
            error!("{:?}",err);
        }
    };
    let pools = reqwest_client.get_pools().await;
    match pools {
        Ok(pools) => report.pools = Some(Pools::new(pools)),
        Err(err) => {
            error!("{:?}",err);
        }
    };

    let volumes = reqwest_client.get_volumes(0).await;
    let volumes  = match volumes {
        Ok(volumes) => Some(volumes),
        Err(err) => {
            error!("{:?}",err);
            None
        }
    };
    let volumes_clone = volumes.clone();

    match volumes{
        Some(volumes) => report.volumes = Some(Volumes::new(volumes)),
        None => {}
    }
    let replicas = reqwest_client.get_replicas().await;
    match replicas {
        Ok(replicas) => report.replicas = Some(Replicas::new(replicas.len(),volumes_clone.clone())),
        Err(err) => {
            error!("{:?}",err);
        }
    };

    let serialized_user = serde_json::to_string(&report).unwrap();
    println!("{}",serialized_user.clone());
    info!("{}",serialized_user);

    Ok(())
}

/*
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub(crate) enum CallHomeError {
    ResourceError(Box<dyn std::error::Error>),
    K8sError(K8sResourceError),
    ReqwestError(ReqwestClientError),
}
impl From<Box<dyn std::error::Error>> for CallHomeError {
    fn from(e: Box<dyn std::error::Error>) -> CallHomeError {
        CallHomeError::ResourceError(e)
    }
}

impl From<K8sResourceError> for CallHomeError {
    fn from(e: K8sResourceErrorr) -> CallHomeError {
        CallHomeError::K8sError(e)
    }
}

impl From<ReqwestClientError> for CallHomeError {
    fn from(e: ReqwestClientError) -> CallHomeError {
        CallHomeError::ReqwestError(e)
    }
}

 */

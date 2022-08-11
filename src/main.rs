mod api_models;
mod report_models;
mod http_client;
mod k8s_client;
use log::{debug, error, log_enabled, info, Level};
use std::{thread, time};
use crate::report_models::{Pools, Replicas, Report, Volumes};
use clap::{App, Arg};
use tokio;
use sha256::digest;
use crate::http_client::{ReqwestClient};
use crate::k8s_client::{K8sClient};

const PRODUCT: &str = "Bolt";


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let matches = App::new(clap::crate_description!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .settings(&[
            clap::AppSettings::ColoredHelp,
            clap::AppSettings::ColorAlways,
        ])
        .arg(
            Arg::with_name("endpoint")
                .long("endpoint")
                .short('e')
                .default_value("http://mayastor-api-rest:8081")
                .help("an URL endpoint to the control plane's rest endpoint"),
        )
        .arg(
            Arg::with_name("namespace")
                .long("namespace")
                .short('n')
                .default_value("mayastor")
                .help("the default namespace we are supposed to operate in"),
        )
        .get_matches();
    let namespace = matches.value_of("namespace").map(|s| s.to_string()).unwrap();
    let endpoint= matches.value_of("endpoint").unwrap();
    let version = clap::crate_version!();

    let k8s_client = K8sClient::new().await.unwrap();
    let reqwest_client = ReqwestClient::new(endpoint).unwrap();

    let k =generate_report(k8s_client.clone(),reqwest_client.clone()).await.unwrap();

    loop{
        let time_to_sleep = time::Duration::from_secs(60);
        thread::sleep(time_to_sleep);
        let k =generate_report(k8s_client.clone(),reqwest_client.clone()).await.unwrap();
    }
}

pub async fn generate_report(k8s_client:K8sClient, reqwest_client : ReqwestClient) -> Result<(), Box<dyn std::error::Error>>
{
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
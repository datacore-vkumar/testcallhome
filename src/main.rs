mod api_models;
mod report_models;
mod http_client;
mod k8s_client;
use log::{debug, error, log_enabled, info, Level};
use std::{thread, time};
use crate::report_models::{Pools, Replicas, Report, Volumes};
use clap::{App, Arg};
use env_logger::Builder;
use tokio;
use sha256::digest;
use crate::http_client::{HttpClient, HttpClientError};
use crate::k8s_client::{K8sClient};

const PRODUCT: &str = "Bolt";


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let matches = App::new(clap::crate_description!())
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
    let version = clap::crate_version!().to_string();

    let k8s_client = K8sClient::new().await.unwrap();
    let http_client = HttpClient::new(endpoint).unwrap();

    let mut report =generate_report(k8s_client.clone(), http_client.clone()).await;
    report.deploy_namespace = namespace.clone();
    report.product_version = version.clone();

    loop{
        let time_to_sleep = time::Duration::from_secs(60);
        thread::sleep(time_to_sleep);
        let mut report =generate_report(k8s_client.clone(), http_client.clone()).await;
        report.deploy_namespace = namespace.clone();
        report.product_version = version.clone();
    }
}

pub async fn generate_report(k8s_client:K8sClient, http_client : HttpClient) -> Report
{
    let mut report = Report::new();
    report.product_name = PRODUCT.to_string();
    let k8s_node_count = k8s_client.get_nodes().await;
    match k8s_node_count {
        Ok(k8s_node_count) => report.k8s_node_count = k8s_node_count as u8,
        Err(err) => {
            error!("{:?}",err);
        }
    };
    let k8s_cluster_id = k8s_client.get_cluster_id().await;
    match k8s_cluster_id {
        Ok(k8s_cluster_id) => report.k8s_cluster_id = digest(k8s_cluster_id),
        Err(err) => {
            error!("{:?}",err);
        }
    };

    let nodes = http_client.get_nodes().await;
    match nodes {
        Ok(nodes) => report.storage_node_count = nodes.len() as u8,
        Err(err) => {
            error!("{:?}",err);
        }
    };
    let pools = http_client.get_pools().await;
    match pools {
        Ok(pools) => report.pools = Pools::new(pools),
        Err(err) => {
            error!("{:?}",err);
        }
    };

    let volumes = http_client.get_volumes(0).await;
    let volumes  = match volumes {
        Ok(volumes) => Some(volumes),
        Err(err) => {
            error!("{:?}",err);
            None
        }
    };

    match volumes.clone() {
        Some(volumes) => report.volumes = Volumes::new(volumes),
        None => {}
    }
    let replicas = http_client.get_replicas().await;
    match replicas {
        Ok(replicas) => report.replicas = Replicas::new(replicas.len(),volumes),
        Err(err) => {
            error!("{:?}",err);
        }
    };

    let serialized_user = serde_json::to_string(&report).unwrap();
    println!("{}",serialized_user.clone());
    info!("{:?}",serialized_user);

    report
}
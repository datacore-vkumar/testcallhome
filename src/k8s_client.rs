use k8s_openapi::api::core::v1::{Node, Namespace};
use kube::{ Api, Client};
use serde_json::Error;
use snafu::Snafu;

///Contains Errors that may generate while execution of k8s_client
#[derive(Debug, Snafu)]
#[allow(clippy::enum_variant_names)]
pub enum K8sResourceError {
    #[snafu(display("Json Parse Error : {}", source))]
    SerdeError {
        source: serde_json::Error,
    },
    #[snafu(display("K8Client Error: {}", source))]
    ClientError {
        source: kube::Error,
    },
}

impl From<kube::Error> for K8sResourceError {
    fn from(source: kube::Error) -> Self {
        Self::ClientError{source}
    }
}

impl From<Error> for K8sResourceError {
    fn from(source: Error) -> Self {
        Self::SerdeError{source}
    }
}

/// K8sClient contains k8s client
#[derive(Clone)]
pub struct K8sClient {
    client: kube::Client
}

impl K8sClient {
    /// Create a new K8sClient from default configuration
    pub(crate) async fn new() -> Result<Self, K8sResourceError> {
        let client = Client::try_default().await?;
        Ok(Self {
            client
        })
    }

    /// Get a clone of the inner `kube::Client`.
    pub(crate) fn kube_client(&self) -> kube::Client {
        self.client.clone()
    }

    ///Get number of  nodes present in the cluster
    pub async fn get_nodes(&self) -> Result<usize, K8sResourceError>
    {
        let nodes: Api<Node> = Api::all(self.client.clone());
        let list = nodes.list(&Default::default()).await?;
        Ok(list.items.len())
    }

    ///Get kube-system namespace uuid
    pub async fn get_cluster_id(&self) -> Result<String, K8sResourceError>
    {
        let namespace_api: Api<Namespace> = Api::all(self.client.clone());
        let kube_system_namespace = namespace_api.get("kube-system").await?;
        let json_object = serde_json::to_value(&kube_system_namespace)?;
        Ok(json_object["metadata"]["uid"].to_string())
    }
}

use k8s_openapi::api::core::v1::{Node, Namespace};
use kube::{ Api, Client};
use serde_json::Error;
use snafu::Snafu;

/*
/// K8sResourceError holds errors that can obtain while fetching
/// information of Kubernetes Objects
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub(crate) enum K8sResourceError {
    ClientConfigError(kube::config::KubeconfigError),
    InferConfigError(kube::config::InferConfigError),
    ClientError(kube::Error),
    ResourceError(Box<dyn std::error::Error>),
    CustomError(String),
    SerdeError(Error)
}

 */
#[derive(Debug, Snafu)]
#[allow(clippy::enum_variant_names)]
pub enum K8sResourceError {
    #[snafu(display(
    "ClientConfigError : {}", source))]
    /// Error generated when the loop stops processing
    ClientConfigError {
        source: kube::config::KubeconfigError,
    },
    #[snafu(display("Json Parse Error : {}", source))]
    SerdeError {
        source: serde_json::Error,
    },
    #[snafu(display("InferConfigError: {}", source))]
    InferConfigError {
        source: kube::config::InferConfigError,
    },
    #[snafu(display("K8Client Error: {}", source))]
    ClientError {
        source: kube::Error,
    },
    Noun {},
}

impl From<kube::config::KubeconfigError> for K8sResourceError {
    fn from(source: kube::config::KubeconfigError) -> Self {
        Self::ClientConfigError{source}
    }
}

impl From<kube::config::InferConfigError> for K8sResourceError {
    fn from(source: kube::config::InferConfigError) -> Self {
        Self::InferConfigError{source}
    }
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

/// ClientSet is wrapper Kubernetes clientset and namespace of mayastor service
#[derive(Clone)]
pub struct K8sClient {
    client: kube::Client
}

impl K8sClient {
    /// Create a new ClientSet, from the config file if provided, otherwise with default.
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

    pub async fn get_nodes(&self) -> Result<usize, K8sResourceError>
    {
        let nodes: Api<Node> = Api::all(self.client.clone());
        let list = nodes.list(&Default::default()).await?;
        Ok(list.items.len())
    }

    pub async fn get_cluster_id(&self) -> Result<String, K8sResourceError>
    {
        let namespace_api: Api<Namespace> = Api::all(self.client.clone());
        let kube_system_namespace = namespace_api.get("kube-system").await?;
        let json_object = serde_json::to_value(&kube_system_namespace)?;
        Ok(json_object["metadata"]["uid"].to_string())
    }
}

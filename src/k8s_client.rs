use k8s_openapi::api::core::v1::{Node, Namespace};
use kube::{api::ListParams, Api, Client, Resource};
use serde_json::Error;

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

impl From<kube::config::KubeconfigError> for K8sResourceError {
    fn from(e: kube::config::KubeconfigError) -> K8sResourceError {
        K8sResourceError::ClientConfigError(e)
    }
}

impl From<kube::config::InferConfigError> for K8sResourceError {
    fn from(e: kube::config::InferConfigError) -> K8sResourceError {
        K8sResourceError::InferConfigError(e)
    }
}
impl From<Box<dyn std::error::Error>> for K8sResourceError {
    fn from(e: Box<dyn std::error::Error>) -> K8sResourceError {
        K8sResourceError::ResourceError(e)
    }
}

impl From<kube::Error> for K8sResourceError {
    fn from(e: kube::Error) -> K8sResourceError {
        K8sResourceError::ClientError(e)
    }
}


impl From<Error> for K8sResourceError {
    fn from(e: Error) -> K8sResourceError {
        K8sResourceError::SerdeError(e)
    }
}

impl From<String> for K8sResourceError {
    fn from(e: String) -> K8sResourceError {
        K8sResourceError::CustomError(e)
    }
}

impl K8sResourceError {
    /// Returns K8sResourceError from provided message
    pub fn invalid_k8s_resource_value(err: String) -> Self {
        Self::CustomError(err)
    }
}

/// ClientSet is wrapper Kubernetes clientset and namespace of mayastor service
#[derive(Clone)]
pub(crate) struct ClientSet {
    kube_config: kube::Config,
    client: kube::Client,
}

impl ClientSet {
    /// Create a new ClientSet, from the config file if provided, otherwise with default.
    pub(crate) async fn new(
        kube_config_path: Option<std::path::PathBuf>
    ) -> Result<Self, K8sResourceError> {
        let config = match kube_config_path {
            Some(config_path) => {
                let kube_config = kube::config::Kubeconfig::read_from(&config_path)
                    .map_err(|e| -> K8sResourceError { e.into() })?;
                kube::Config::from_custom_kubeconfig(kube_config, &Default::default()).await?
            }
            None => kube::Config::infer().await?,
        };
        let client = Client::try_from(config.clone())?;
        Ok(Self {
            client,
            kube_config: config,
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




















/*use kube::{Api, Client, Error};
use k8s_openapi::api::core::v1::Node;
use k8s_openapi::api::core::v1::Namespace;

pub struct K8sClient {
    client: Client,
}

impl K8sClient {
    pub async fn new() -> Result<Self, Error>
    {
        let k8s_client = Client::try_default().await?;
        Ok(Self {
            client: k8s_client,
        })
    }
    pub async fn get_nodes(&self) -> Option<usize>
    {
        let nodes: Api<Node> = Api::all(self.client.clone());
        let list = nodes.list(&Default::default()).await;
        match list {
            Ok(list) => Some(list.items.len()),
            Err(err) => {
                println!("{:?}",err);
                None
            }
        }
    }

    pub async fn get_cluster_id(&self) -> Option<String>
    {
        let namespace_api: Api<Namespace> = Api::all(self.client.clone());
        let kube_system_namespace = namespace_api.get("kube-system").await.unwrap();
        let json_object = serde_json::to_value(&kube_system_namespace);
        match json_object {
            Ok(json_object) => Some(json_object["metadata"]["uid"].to_string()),
            Err(err) => {
                println!("{:?}",err);
                None
            }
        }
    }
}

 */
use serde_json::Value;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VolumesApi
{
    pub entries : Vec<VolumeStats>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VolumeStats
{
    pub spec : VolumeSpec
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct VolumeSpec {
    pub num_replicas : u64,
    pub size :	u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolsApi
{
    pub state : PoolsState
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolsState
{
    pub capacity: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodesApi
{
    pub id : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplicasApi
{
   pub node : String,
}
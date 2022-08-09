use serde_json::Value;
use serde::Serialize;
use serde::Deserialize;
use crate::api_models::{PoolsApi, PoolsState, VolumesApi, VolumeStats};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Volumes
{
    pub count: u64,
    pub max_size_in_bytes: u64,
    pub min_size_in_bytes: u64,
    pub mean_size_in_bytes: f64,
    pub capacity_percentiles_in_bytes: Percentiles
}
impl Volumes {
    pub(crate) fn default() -> Self
    {
        Self {
            count: 0,
            mean_size_in_bytes: 0.0,
            min_size_in_bytes: 0,
            max_size_in_bytes: 0,
            capacity_percentiles_in_bytes: Percentiles::default(),
        }
    }
   pub(crate) fn new(mut volume_entries:VolumesApi) -> Self
    {
        let mut volumes_vector = convert_volumes_into_vector(volume_entries.entries);
        if(volumes_vector.len() > 0)
        {
            return Self {
                count: volumes_vector.len() as u64,
                max_size_in_bytes: find_max(volumes_vector.clone()),
                min_size_in_bytes: find_min(volumes_vector.clone()),
                mean_size_in_bytes: find_mean(volumes_vector.clone()),
                capacity_percentiles_in_bytes: Percentiles::new(volumes_vector.clone()),
            };
        }
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pools
{
    pub count: u64,
    pub max_size_in_bytes: u64,
    pub min_size_in_bytes: u64,
    pub mean_size_in_bytes: f64,
    pub capacity_percentiles_in_bytes: Percentiles
}
impl Pools {
    pub(crate) fn default() -> Self
    {
        Self {
            count: 0,
            max_size_in_bytes: 0,
            min_size_in_bytes: 0,
            mean_size_in_bytes: 0.0,
            capacity_percentiles_in_bytes: Percentiles::default()
        }
    }
    pub(crate) fn new(pools:Vec<PoolsApi>) -> Self
    {
        let mut pools_vector = convert_pools_into_vector(pools);
        if(pools_vector.len() > 0)
        {
            return Self {
                count: pools_vector.len() as u64,
                max_size_in_bytes: find_max(pools_vector.clone()),
                min_size_in_bytes: find_min(pools_vector.clone()),
                mean_size_in_bytes: find_mean(pools_vector.clone()),
                capacity_percentiles_in_bytes: Percentiles::new(pools_vector.clone())
            };
        }
        Self::default()
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Replicas
{
     count: u64,
     count_per_volume_percentiles: Percentiles,
}
impl Replicas {
    pub fn default() -> Self
    {
        Self {
            count: 0,
            count_per_volume_percentiles: Percentiles::default(),
        }
    }
    pub fn new(replica_count : usize, mut volumes: Option<VolumesApi>) -> Self
    {
        let mut replicas = Self::default();
        match volumes {
            Some(volumes) => {
                let volumes_vector = convert_replicas_into_vector(volumes.entries);
                if volumes_vector.len()>0
                {
                    replicas.count_per_volume_percentiles = Percentiles::new(volumes_vector.clone());
                }
                else
                {
                    replicas.count_per_volume_percentiles = Percentiles::new(volumes_vector.clone());
                }
            }
            None => {}
        };
        replicas.count = replica_count as u64;
        replicas
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Versions
{
    control_plane_version: String,
}
impl Versions {
    pub(crate) fn new() -> Self
    {
        Self {
            control_plane_version: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Percentiles
{
    #[serde(rename = "50%")]
    pub p50 : u64,
    #[serde(rename = "75%")]
    pub p75 : u64,
    #[serde(rename = "90%")]
    pub p90 : u64,
}

impl Percentiles {
    pub(crate) fn default() -> Self
    {
        Self {
            p50: 0,
            p75: 0,
            p90: 0,
        }
    }

    pub(crate) fn new(values: Vec<u64>) -> Self
    {
        Self {
            p50: find_percentiles(values.clone(), 50),
            p75: find_percentiles(values.clone(), 75),
            p90: find_percentiles(values.clone(), 90),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Report
{
    pub k8s_cluster_id:Option<String>,
    pub k8s_node_count: Option<u64>,
    pub product_name: Option<String>,
    pub product_version: Option<String>,
    pub deploy_namespace: Option<String>,
    pub storage_node_count:Option<u64>,
    pub pools : Option<Pools>,
    pub volumes : Option<Volumes>,
    pub replicas: Option<Replicas>,
    pub versions : Option<Versions>,
}
impl Report
{
    pub(crate) fn new() -> Self
    {
        Self{
            k8s_cluster_id: None,
            k8s_node_count: None,
            product_name: None,
            product_version: None,
            deploy_namespace: None,
            storage_node_count: None,
            pools: None,
            volumes: None,
            replicas: None,
            versions: None,
        }
    }
}

fn find_max(values: Vec<u64>) -> u64
{
    let mut max_value = values.iter().max().unwrap();
    *max_value
}
fn find_min(values : Vec<u64>) -> u64
{
    let mut min_value = values.iter().min().unwrap();
    *min_value
}
fn find_mean(values : Vec<u64>) -> f64
{
    let mut sum= 0.0;
    for value in values.iter() {
        sum += *value as f64/(values.len() as f64);
    }
    sum
}

fn find_percentiles(mut values: Vec<u64>, percentile : usize) -> u64
{
    values.sort();
    if((percentile * values.len()) % 100 == 0)
    {
        let index = (percentile*values.len()/100);
        if(index > 0)
        {
            values[index - 1]
        }
        else {
            values[index]
        }
    }
    else {
        let index = (percentile*values.len())/100;
        if(index > 0)
        {
            return (values[index] + values[index-1])/2;
        }
        values[index]
    }
}
fn convert_volumes_into_vector(volumes: Vec<VolumeStats>) -> Vec<u64>
{
    let mut volume_vector = Vec::new();
    for f in volumes.iter() {
        volume_vector.push(f.spec.size);
    }
    volume_vector
}
fn convert_replicas_into_vector(volumes: Vec<VolumeStats>) -> Vec<u64>
{
    let mut replicas_vector = Vec::new();
    for volume in volumes.iter() {
        replicas_vector.push(volume.spec.num_replicas);
    }
    replicas_vector
}
fn convert_pools_into_vector(pools: Vec<PoolsApi>) -> Vec<u64>
{
    let mut pools_vector = Vec::new();
    for pool in pools.iter() {
        pools_vector.push(pool.state.capacity);
    }
    pools_vector
}
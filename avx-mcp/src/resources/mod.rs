pub mod aviladb;
pub mod avx_config;
pub mod cluster;

use crate::protocol::Resource;

pub fn get_all_resources() -> Vec<Resource> {
    let mut resources = Vec::new();
    resources.extend(aviladb::get_aviladb_resources());
    resources.extend(avx_config::get_avx_config_resources());
    resources.extend(cluster::get_cluster_resources());
    resources
}

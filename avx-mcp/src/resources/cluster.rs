use crate::protocol::Resource;

pub fn get_cluster_resources() -> Vec<Resource> {
    vec![
        Resource {
            uri: "avx://cluster/production".to_string(),
            name: "Production Cluster".to_string(),
            description: Some("Production Kubernetes cluster".to_string()),
            mime_type: Some("application/json".to_string()),
        },
        Resource {
            uri: "avx://cluster/staging".to_string(),
            name: "Staging Cluster".to_string(),
            description: Some("Staging Kubernetes cluster".to_string()),
            mime_type: Some("application/json".to_string()),
        },
    ]
}

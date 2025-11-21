use crate::protocol::Resource;

pub fn get_avx_config_resources() -> Vec<Resource> {
    vec![
        Resource {
            uri: "avx://config/stack".to_string(),
            name: "AVX Stack Config".to_string(),
            description: Some("Current stack configuration".to_string()),
            mime_type: Some("application/json".to_string()),
        },
        Resource {
            uri: "avx://config/mesh".to_string(),
            name: "AVX Mesh Config".to_string(),
            description: Some("Service mesh configuration".to_string()),
            mime_type: Some("application/json".to_string()),
        },
    ]
}

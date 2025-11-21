use crate::protocol::Resource;

pub fn get_aviladb_resources() -> Vec<Resource> {
    vec![
        Resource {
            uri: "aviladb://production/users".to_string(),
            name: "Users Database".to_string(),
            description: Some("Production user database collection".to_string()),
            mime_type: Some("application/json".to_string()),
        },
        Resource {
            uri: "aviladb://production/events".to_string(),
            name: "Events Database".to_string(),
            description: Some("Production events collection".to_string()),
            mime_type: Some("application/json".to_string()),
        },
    ]
}

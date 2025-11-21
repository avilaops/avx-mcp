use crate::protocol::Tool;
use serde_json::json;

pub fn get_deploy_tool() -> Tool {
    Tool {
        name: "avx_deploy".to_string(),
        description: "Deploy services to AVX clusters with automatic K8s manifest generation".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "service": {
                    "type": "string",
                    "enum": ["gateway", "api-core", "events"],
                    "description": "Service to deploy"
                },
                "cluster": {
                    "type": "string",
                    "description": "Target cluster (e.g., 'production', 'staging')"
                },
                "namespace": {
                    "type": "string",
                    "description": "Kubernetes namespace",
                    "default": "avx-core"
                },
                "replicas": {
                    "type": "number",
                    "description": "Number of replicas",
                    "default": 2
                },
                "image": {
                    "type": "string",
                    "description": "Container image (optional, uses default if not specified)"
                }
            },
            "required": ["service", "cluster"]
        }),
    }
}

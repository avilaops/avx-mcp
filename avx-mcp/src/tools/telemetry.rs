use crate::protocol::Tool;
use serde_json::json;

pub fn get_telemetry_tool() -> Tool {
    Tool {
        name: "avx_telemetry".to_string(),
        description: "Get real-time telemetry and metrics from AVX services".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "service": {
                    "type": "string",
                    "description": "Service name (e.g., 'gateway', 'api-core')"
                },
                "metric": {
                    "type": "string",
                    "enum": ["cpu", "memory", "requests", "latency", "errors"],
                    "description": "Metric type to retrieve"
                },
                "cluster": {
                    "type": "string",
                    "description": "Target cluster",
                    "default": "production"
                },
                "timeRange": {
                    "type": "string",
                    "description": "Time range (e.g., '1h', '24h', '7d')",
                    "default": "1h"
                }
            },
            "required": ["service", "metric"]
        }),
    }
}

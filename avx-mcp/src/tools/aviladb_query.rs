use crate::protocol::Tool;
use serde_json::json;

pub fn get_query_tool() -> Tool {
    Tool {
        name: "avx_query".to_string(),
        description: "Query AvilaDB collections with filters and projections".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "database": {
                    "type": "string",
                    "description": "Database name (e.g., 'production', 'staging')"
                },
                "collection": {
                    "type": "string",
                    "description": "Collection name (e.g., 'users', 'events')"
                },
                "query": {
                    "type": "string",
                    "description": "JSON query filter"
                },
                "limit": {
                    "type": "number",
                    "description": "Maximum number of results",
                    "default": 100
                }
            },
            "required": ["database", "collection", "query"]
        }),
    }
}

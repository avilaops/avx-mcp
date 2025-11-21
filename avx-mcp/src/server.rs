use crate::protocol::*;
use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub struct McpServer {
    initialized: bool,
    resources: Vec<Resource>,
    tools: Vec<Tool>,
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            initialized: false,
            resources: Vec::new(),
            tools: Vec::new(),
        }
    }

    pub fn register_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }

    pub fn register_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("🚀 AVX MCP Server starting...");

        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;

            if n == 0 {
                break; // EOF
            }

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            tracing::debug!("Received: {}", line);

            let request: JsonRpcRequest = match serde_json::from_str(line) {
                Ok(req) => req,
                Err(e) => {
                    tracing::error!("Failed to parse request: {}", e);
                    continue;
                }
            };

            let response = self.handle_request(request).await;
            let response_json = serde_json::to_string(&response)?;

            tracing::debug!("Sending: {}", response_json);

            stdout.write_all(response_json.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
        }

        tracing::info!("👋 AVX MCP Server shutting down");
        Ok(())
    }

    async fn handle_request(&mut self, request: JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request),
            "initialized" => self.handle_initialized(request),
            "resources/list" => self.handle_resources_list(request),
            "resources/read" => self.handle_resources_read(request),
            "tools/list" => self.handle_tools_list(request),
            "tools/call" => self.handle_tools_call(request).await,
            _ => JsonRpcResponse::error(
                request.id,
                -32601,
                format!("Method not found: {}", request.method),
            ),
        }
    }

    fn handle_initialize(&mut self, request: JsonRpcRequest) -> JsonRpcResponse {
        let result = InitializeResult {
            protocol_version: PROTOCOL_VERSION.to_string(),
            capabilities: ServerCapabilities {
                resources: Some(ResourceCapabilities {
                    subscribe: Some(false),
                    list_changed: Some(false),
                }),
                tools: Some(ToolCapabilities {
                    list_changed: Some(false),
                }),
                prompts: None,
            },
            server_info: ServerInfo {
                name: "avx-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        JsonRpcResponse::success(request.id, json!(result))
    }

    fn handle_initialized(&mut self, request: JsonRpcRequest) -> JsonRpcResponse {
        self.initialized = true;
        tracing::info!("✅ MCP Server initialized");
        JsonRpcResponse::success(request.id, json!({}))
    }

    fn handle_resources_list(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let result = json!({
            "resources": self.resources
        });
        JsonRpcResponse::success(request.id, result)
    }

    fn handle_resources_read(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let params: HashMap<String, serde_json::Value> = match request.params {
            Some(p) => serde_json::from_value(p).unwrap_or_default(),
            None => HashMap::new(),
        };

        let uri = params
            .get("uri")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Mock resource content
        let contents = ResourceContents {
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
            text: Some(json!({"status": "ok", "uri": uri}).to_string()),
            blob: None,
        };

        let result = json!({
            "contents": [contents]
        });

        JsonRpcResponse::success(request.id, result)
    }

    fn handle_tools_list(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let result = json!({
            "tools": self.tools
        });
        JsonRpcResponse::success(request.id, result)
    }

    async fn handle_tools_call(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let params: CallToolParams = match request.params {
            Some(p) => match serde_json::from_value(p) {
                Ok(params) => params,
                Err(e) => {
                    return JsonRpcResponse::error(
                        request.id,
                        -32602,
                        format!("Invalid params: {}", e),
                    )
                }
            },
            None => {
                return JsonRpcResponse::error(request.id, -32602, "Missing params".to_string())
            }
        };

        // Execute tool based on name
        let result_text = match params.name.as_str() {
            "avx_query" => self.execute_query(&params.arguments).await,
            "avx_deploy" => self.execute_deploy(&params.arguments).await,
            "avx_telemetry" => self.execute_telemetry(&params.arguments).await,
            _ => format!("Unknown tool: {}", params.name),
        };

        let result = CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        };

        JsonRpcResponse::success(request.id, json!(result))
    }

    async fn execute_query(&self, args: &HashMap<String, serde_json::Value>) -> String {
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        format!("🔍 Executed query: {}", query)
    }

    async fn execute_deploy(&self, args: &HashMap<String, serde_json::Value>) -> String {
        let service = args
            .get("service")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        format!("🚀 Deployed service: {}", service)
    }

    async fn execute_telemetry(&self, args: &HashMap<String, serde_json::Value>) -> String {
        let metric = args
            .get("metric")
            .and_then(|v| v.as_str())
            .unwrap_or("cpu");
        format!("📊 Telemetry for {}: 42%", metric)
    }
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

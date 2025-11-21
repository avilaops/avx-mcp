use anyhow::Result;
use avx_mcp::{McpServer, get_all_resources, get_all_tools};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "avx_mcp=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    // Create and configure server
    let mut server = McpServer::new();

    // Register all resources
    for resource in get_all_resources() {
        server.register_resource(resource);
    }

    // Register all tools
    for tool in get_all_tools() {
        server.register_tool(tool);
    }

    // Run server
    server.run().await?;

    Ok(())
}

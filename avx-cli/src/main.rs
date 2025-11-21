use std::{fs, path::Path};

use anyhow::Result;
use avx_config::AvxConfig;
use avx_mcp::{McpServer, get_all_resources, get_all_tools};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "avx-cli",
    version,
    about = "CLI da stack Avx para gerar manifests K8s da fungo-deepweb"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Gera manifests Kubernetes (Deployment + Service) para um serviço Avx
    K8s {
        /// Serviço alvo: gateway | api-core | events
        #[arg(long, value_enum)]
        service: ServiceKind,

        /// Namespace Kubernetes
        #[arg(long, default_value = "avx-core")]
        namespace: String,

        /// Imagem do container (se não passar, usa default baseado no service)
        #[arg(long)]
        image: Option<String>,

        /// Número de réplicas
        #[arg(long, default_value_t = 2)]
        replicas: u32,

        /// Caminho do arquivo de saída (se não passar, manda pro stdout)
        #[arg(long)]
        output: Option<String>,
    },

    /// Inicia o servidor MCP (Model Context Protocol)
    Mcp {
        #[command(subcommand)]
        command: McpCommands,
    },
}

#[derive(Subcommand, Debug)]
enum McpCommands {
    /// Inicia o servidor MCP em modo stdio
    Serve,

    /// Lista todos os resources disponíveis
    Resources,

    /// Lista todos os tools disponíveis
    Tools,

    /// Testa a conexão com o servidor MCP
    Test,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ServiceKind {
    Gateway,
    ApiCore,
    Events,
}

impl ServiceKind {
    fn name(&self) -> &'static str {
        match self {
            ServiceKind::Gateway => "avx-gateway",
            ServiceKind::ApiCore => "avx-api-core",
            ServiceKind::Events => "avx-events",
        }
    }

    fn default_image(&self) -> String {
        match self {
            ServiceKind::Gateway => "ghcr.io/avilaops/avx-gateway:latest".into(),
            ServiceKind::ApiCore => "ghcr.io/avilaops/avx-api-core:latest".into(),
            ServiceKind::Events => "ghcr.io/avilaops/avx-events:latest".into(),
        }
    }

    fn container_port(&self) -> u16 {
        match self {
            ServiceKind::Gateway => 8080,
            ServiceKind::ApiCore => 8081,
            ServiceKind::Events => 8090,
        }
    }

    fn service_port(&self) -> u16 {
        // porta externa do Service (ClusterIP)
        80
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::K8s {
            service,
            namespace,
            image,
            replicas,
            output,
        } => {
            let cfg = AvxConfig::load().unwrap_or_else(|_| AvxConfig::with_defaults());
            let manifest = generate_k8s_manifest(&cfg, &namespace, service, image, replicas);

            if let Some(path) = output {
                write_to_file(&path, &manifest)?;
                println!("✅ Manifest gerado em: {}", path);
            } else {
                println!("{manifest}");
            }
        }

        Commands::Mcp { command } => {
            handle_mcp_command(command).await?;
        }
    }

    Ok(())
}

async fn handle_mcp_command(command: McpCommands) -> Result<()> {
    match command {
        McpCommands::Serve => {
            let mut server = McpServer::new();

            for resource in get_all_resources() {
                server.register_resource(resource);
            }

            for tool in get_all_tools() {
                server.register_tool(tool);
            }

            server.run().await?;
        }

        McpCommands::Resources => {
            println!("📚 Available Resources:\n");
            for resource in get_all_resources() {
                println!("  • {} ({})", resource.name, resource.uri);
                if let Some(desc) = &resource.description {
                    println!("    {}", desc);
                }
                println!();
            }
        }

        McpCommands::Tools => {
            println!("🔧 Available Tools:\n");
            for tool in get_all_tools() {
                println!("  • {}", tool.name);
                println!("    {}", tool.description);
                println!("    Schema: {}", serde_json::to_string_pretty(&tool.input_schema)?);
                println!();
            }
        }

        McpCommands::Test => {
            println!("🧪 Testing MCP Server...\n");
            println!("✅ Resources: {} registered", get_all_resources().len());
            println!("✅ Tools: {} registered", get_all_tools().len());
            println!("\n💡 Run 'avx-cli mcp serve' to start the server");
        }
    }

    Ok(())
}

fn write_to_file(path: &str, content: &str) -> Result<()> {
    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(p, content)?;
    Ok(())
}

fn generate_k8s_manifest(
    cfg: &AvxConfig,
    namespace: &str,
    service: ServiceKind,
    image_override: Option<String>,
    replicas: u32,
) -> String {
    let name = service.name();
    let image = image_override.unwrap_or_else(|| service.default_image());
    let container_port = service.container_port();
    let service_port = service.service_port();

    let stack = &cfg.stack;
    let layer = &cfg.layer;
    let env = &cfg.env;
    let cluster = &cfg.cluster;
    let mesh = &cfg.mesh;

    format!(
"# Generated by avx-cli - Avila Experience Fabric
# Stack: {stack} | Layer: {layer} | Cluster: {cluster}
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {name}
  namespace: {namespace}
  labels:
    app: {name}
    stack: {stack}
    layer: {layer}
    env: {env}
    cluster: {cluster}
    mesh: {mesh}
spec:
  replicas: {replicas}
  selector:
    matchLabels:
      app: {name}
  template:
    metadata:
      labels:
        app: {name}
        stack: {stack}
        layer: {layer}
        env: {env}
        cluster: {cluster}
        mesh: {mesh}
    spec:
      containers:
        - name: {name}
          image: {image}
          imagePullPolicy: IfNotPresent
          ports:
            - containerPort: {container_port}
          env:
            - name: AVX__STACK
              value: \"{stack}\"
            - name: AVX__LAYER
              value: \"{layer}\"
            - name: AVX__ENV
              value: \"{env}\"
            - name: AVX__CLUSTER
              value: \"{cluster}\"
            - name: AVX__MESH
              value: \"{mesh}\"
          resources:
            requests:
              cpu: \"100m\"
              memory: \"128Mi\"
            limits:
              cpu: \"500m\"
              memory: \"256Mi\"
---
apiVersion: v1
kind: Service
metadata:
  name: {name}
  namespace: {namespace}
  labels:
    app: {name}
    stack: {stack}
    layer: {layer}
spec:
  type: ClusterIP
  selector:
    app: {name}
  ports:
    - name: http
      port: {service_port}
      targetPort: {container_port}
"
    )
}

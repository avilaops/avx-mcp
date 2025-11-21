use serde::{Deserialize, Serialize};
use std::env;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvxConfig {
    pub stack: String,
    pub layer: String,
    pub env: String,
    pub cluster: String,
    pub mesh: String,
}

impl AvxConfig {
    pub fn load() -> Result<Self> {
        Ok(Self {
            stack: env::var("AVX__STACK").unwrap_or_else(|_| "default".into()),
            layer: env::var("AVX__LAYER").unwrap_or_else(|_| "core".into()),
            env: env::var("AVX__ENV").unwrap_or_else(|_| "dev".into()),
            cluster: env::var("AVX__CLUSTER").unwrap_or_else(|_| "local".into()),
            mesh: env::var("AVX__MESH").unwrap_or_else(|_| "default".into()),
        })
    }

    pub fn with_defaults() -> Self {
        Self {
            stack: "default".into(),
            layer: "core".into(),
            env: "dev".into(),
            cluster: "local".into(),
            mesh: "default".into(),
        }
    }
}

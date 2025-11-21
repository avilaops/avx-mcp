pub mod aviladb_query;
pub mod deploy;
pub mod telemetry;

use crate::protocol::Tool;

pub fn get_all_tools() -> Vec<Tool> {
    vec![
        aviladb_query::get_query_tool(),
        deploy::get_deploy_tool(),
        telemetry::get_telemetry_tool(),
    ]
}

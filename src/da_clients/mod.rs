use crate::jobs::types::JobVerificationStatus;
use axum::async_trait;
use color_eyre::Result;
use starknet::core::types::FieldElement;

/// Ethereum client
pub mod ethereum;

/// Trait for every new DaClient to implement
#[async_trait]
pub trait DaClient: Send + Sync {
    /// Should publish the state diff to the DA layer and return an external id
    /// which can be used to track the status of the DA transaction.
    async fn publish_state_diff(&self, state_diff: Vec<FieldElement>) -> Result<String>;
    /// Should verify the inclusion of the state diff in the DA layer and return the status
    async fn verify_inclusion(&self, external_id: &str) -> Result<JobVerificationStatus>;
}

/// Trait for every new DaConfig to implement
pub trait DaConfig {
    /// Should create a new instance of the DaConfig from the environment variables
    fn new_from_env() -> Self;
}

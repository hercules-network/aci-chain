pub mod hadron;
pub mod quark;

// --- std ---
use std::sync::Arc;
// --- substrate ---
use sc_keystore::LocalKeystore;
pub use sc_service::ChainSpec;
use sc_service::{config::PrometheusConfig, Configuration, Error as ServiceError};
use sp_runtime::traits::BlakeTwo256;
use substrate_prometheus_endpoint::Registry;

use aci_primitives::OpaqueBlock as Block;

pub type FullClient<RuntimeApi, Executor> = sc_service::TFullClient<Block, RuntimeApi, Executor>;
pub type FullBackend = sc_service::TFullBackend<Block>;
pub type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
// type FullGrandpaBlockImport<RuntimeApi, Executor> = sc_finality_grandpa::GrandpaBlockImport<
//     FullBackend,
//     Block,
//     FullClient<RuntimeApi, Executor>,
//     FullSelectChain,
// >;
pub type LightBackend = sc_service::TLightBackendWithHash<Block, BlakeTwo256>;
pub type LightClient<RuntimeApi, Executor> =
    sc_service::TLightClientWithBackend<Block, RuntimeApi, Executor, LightBackend>;

/// Can be called for a `Configuration` to check if it is a configuration for the `quark` chain.
pub trait IdentifyVariant {
    /// Returns if this is a configuration for the `Quark` chain.
    fn is_quark_network(&self) -> bool;

    /// Returns if this is a configuration for the `Hadron` chain.
    fn is_hadron_network(&self) -> bool;
}
impl IdentifyVariant for Box<dyn ChainSpec> {
    fn is_quark_network(&self) -> bool {
        self.id().starts_with("quark") || self.id().starts_with("lyr")
    }

    fn is_hadron_network(&self) -> bool {
        self.id().starts_with("hadron")
    }
}

// If we're using prometheus, use a registry with a prefix of `asterism`.
fn set_prometheus_registry(config: &mut Configuration) -> Result<(), ServiceError> {
    if let Some(PrometheusConfig { registry, .. }) = config.prometheus_config.as_mut() {
        *registry = Registry::new_custom(Some("aci".into()), None)?;
    }

    Ok(())
}

fn remote_keystore(_url: &String) -> Result<Arc<LocalKeystore>, &'static str> {
    // FIXME: here would the concrete keystore be built,
    //        must return a concrete type (NOT `LocalKeystore`) that
    //        implements `CryptoStore` and `SyncCryptoStore`
    Err("Remote Keystore not supported.")
}
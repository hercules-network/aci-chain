//! Aci chain configurations.

pub mod quark;
pub use quark::{
    quark_config, quark_development_config, quark_local_testnet_config, staging_config,
    QuarkChainSpec,
};

pub mod hadron;
pub use hadron::{
    hadron_config, hadron_development_config, hadron_local_testnet_config, hadron_staging_config,
    hadron_testnet_config, HadronChainSpec,
};

// --- crates ---
use serde::{Deserialize, Serialize};
// --- substrate ---
use aci_primitives::{AccountId, AccountPublic};
use sc_chain_spec::ChainSpecExtension;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::IdentifyAccount;

const DEFAULT_PROTOCOL_ID: &str = "aci";
const TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<aci_primitives::OpaqueBlock>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<aci_primitives::OpaqueBlock>,
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn testnet_accounts() -> Vec<AccountId> {
    vec![
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        get_account_id_from_seed::<sr25519::Public>("Bob"),
        get_account_id_from_seed::<sr25519::Public>("Charlie"),
        get_account_id_from_seed::<sr25519::Public>("Dave"),
        get_account_id_from_seed::<sr25519::Public>("Eve"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
        get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
        get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
        get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
        get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
    ]
}

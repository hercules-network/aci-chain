use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public, H160, U256};
use std::collections::BTreeMap;

use super::testnet_accounts;
use aci_hadron_runtime::constants::currency::*;
use aci_hadron_runtime::{
    get_all_module_accounts, opaque::SessionKeys, BalancesConfig, BridgeConfig, ContractsConfig,
    CouncilMembershipConfig, CurrencyId, EVMConfig, EthereumConfig, GenesisConfig, SessionConfig,
    Signature, SudoConfig, SystemConfig, TechnicalMembershipConfig, UniTokensConfig,
    ValidatorSetConfig, VestingConfig, WASM_BINARY,
};
use aci_primitives::{AccountId, Balance};

use hex_literal::hex;
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::str::FromStr;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type HadronChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, AuraId, GrandpaId) {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s),
    )
}

pub fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
    SessionKeys { aura, grandpa }
}

pub fn properties() -> Option<Properties> {
    let properties = serde_json::json!({
        "ss58Format": 2,
        "tokenDecimals": 12,
        "tokenSymbol": "ACI",
        "uinkDecimals": 12,
        "uinkSymbol": "UINK",
    });

    serde_json::from_value(properties).ok()
}

pub fn hadron_config() -> Result<HadronChainSpec, String> {
    HadronChainSpec::from_json_bytes(&include_bytes!("../../res/hadron.json")[..])
}

pub fn hadron_testnet_config() -> Result<HadronChainSpec, String> {
    HadronChainSpec::from_json_bytes(&include_bytes!("../../res/hadron_test.json")[..])
}

pub fn hadron_staging_config() -> Result<HadronChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Staging wasm binary not available".to_string())?;

    let initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId)> = vec![(
        hex!("fcbe30f9ea858f3cc334772c399976c5c86506944700bcad21f7dc92dde6d173").into(),
        hex!("fcbe30f9ea858f3cc334772c399976c5c86506944700bcad21f7dc92dde6d173").into(),
        hex!("fcbe30f9ea858f3cc334772c399976c5c86506944700bcad21f7dc92dde6d173").unchecked_into(),
        hex!("d5d815e2f928126c5acc3ad3424f790cd5f2512b2ce75b4001d74e17d0149fae").unchecked_into(),
    )];

    let endowed_accounts: Vec<(AccountId, Balance)> = vec![
        // Parachain award 20%
        (
            hex!("90e8e5d81d429880185694c4175b0720d2c336aedf026d3f47b05643fcad6a2d").into(),
            20_000_000 * ACI,
        ),
        // Team 20%
        (
            hex!("22818069e4959c3130ae728ddb532dad9c27f179c64a0fe4de1ba2ef15035841").into(),
            20_000_000 * ACI,
        ),
        // Token sale 26%
        (
            hex!("9cb78b6b82c7007f6bf4f55d95535d10f98a3c6cbbb27f5f216c9c030bab983f").into(),
            26_000_000 * ACI,
        ),
        // Treasury 20%
        (
            hex!("58390913ec4a2371fee5bc8a81d1fd6d500012adc0f408f1aa56267c0414501b").into(),
            20_000_000 * ACI,
        ),
        // Adivsors 6%
        (
            hex!("827287f438297be6947dce38caf80ddbf750fa0db16a18bbf77cbc6a1e5a551e").into(),
            6_000_000 * ACI,
        ),
        // Ecosystem & Artists %8
        (
            hex!("ac2d4a632aedf3157f1c872d0778b3a5f75f38622f36f53e93025963acb6b709").into(),
            8_000_000 * ACI,
        ),
    ];

    let council_members: Vec<AccountId> = vec![
        hex!("267a5109828e42a59a685d2572fb45c6b0bac9f0173529f3fa0bee5460d56441").into(),
        hex!("64074ed3b0fdabf3d69081b0c629a0d6e769e1ae55f6b7a57a52a57f93566d2b").into(),
        hex!("4ea6722ad3b807d90729fdb1c0669ea14689d3578bb8fec283c991cb36eafb2c").into(),
        hex!("9c588a20b7522a5b13d37f9408f306e14f1b9e0ab2b0cca58d175b61286d4e76").into(),
    ];

    let technical_members: Vec<AccountId> = vec![
        hex!("64074ed3b0fdabf3d69081b0c629a0d6e769e1ae55f6b7a57a52a57f93566d2b").into(),
        hex!("4ea6722ad3b807d90729fdb1c0669ea14689d3578bb8fec283c991cb36eafb2c").into(),
        hex!("9c588a20b7522a5b13d37f9408f306e14f1b9e0ab2b0cca58d175b61286d4e76").into(),
        hex!("221839b0f1bdf567f1ec34f1c4fb6b40cb85ce459d1b26692efca2636f006923").into(),
        hex!("8650d8cace6a60ee5002791b3a3c869e172ecb58f8200769ea16d111ad63ba00").into(),
        hex!("58b1bcd4caee91d2e83acc84862e88e4649d7fdec6bb232c2c6f2406b8ba3e51").into(),
        hex!("34d1df070e06a5cec61b4d63e627d694270466baa7d31e2115adda2139582a0c").into(),
        hex!("0a7736fabb656fcab49dc4a6c0ab0906057ecb7bf389a2ba407fd35bda8ac97a").into(),
    ];

    let sudo_key: AccountId =
        hex!("4447c1a629b6261a461eed4355f47d30658fb1f8ee20cf5839ab1893d98b4f5e").into();

    Ok(HadronChainSpec::from_genesis(
        // Name
        "Aci Staging network",
        // ID
        "hadron_staging",
        ChainType::Live,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                initial_authorities.clone(),
                // Sudo account
                sudo_key.clone(),
                // Pre-funded accounts
                endowed_accounts.clone(),
                council_members.clone(),
                technical_members.clone(),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        Some(
            TelemetryEndpoints::new(vec![(super::TELEMETRY_URL.to_string(), 0)])
                .expect("telemetry url is valid; qed"),
        ),
        // Protocol ID
        Some(super::DEFAULT_PROTOCOL_ID),
        // Properties
        properties(),
        // Extensions
        None,
    ))
}

pub fn hadron_development_config() -> Result<HadronChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(HadronChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "hadron_dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ]
                .iter()
                .map(|k| (k.clone(), 100_000 * ACI))
                .collect::<Vec<_>>(),
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        properties(),
        // Extensions
        None,
    ))
}

pub fn hadron_local_testnet_config() -> Result<HadronChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;
    let endowed_accounts: Vec<AccountId> = testnet_accounts();

    Ok(HadronChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "hadron_local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                endowed_accounts
                    .iter()
                    .map(|k| (k.clone(), 100_000 * ACI))
                    .chain(
                        get_all_module_accounts()
                            .iter()
                            .map(|x| (x.clone(), 100_000_000 * ACI)),
                    )
                    .collect::<Vec<_>>(),
                vec![],
                vec![],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        properties(),
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<(AccountId, Balance)>,
    council_members: Vec<AccountId>,
    technical_members: Vec<AccountId>,
    enable_println: bool,
) -> GenesisConfig {
    let bridge_validators = vec![
        hex!("c0c4ffc36cec38e37f68fa34dd6c14dc3788841b80372981ff2a46583cdea50b").into(),
        hex!("a6159859c000019b92a858037baed672ba50e50e658e9f90a00d00c86e3fe438").into(),
        hex!("2abf312c5a9300f91485abb555597c8305e378d16017316a8303a275f8cee245").into(),
        // get_account_id_from_seed::<sr25519::Public>("Alice"),
        // get_account_id_from_seed::<sr25519::Public>("Bob"),
        // get_account_id_from_seed::<sr25519::Public>("Charlie"),
    ];

    let gen_evm_account_id = H160::from_str("7184035beEAD581f3dcEDeCBa5dB6547A914fBB9").unwrap();
    let mut evm_accounts = BTreeMap::new();
    evm_accounts.insert(
        gen_evm_account_id,
        pallet_evm::GenesisAccount {
            nonce: 0.into(),
            balance: U256::from(1000000000_000_000_000_000_000_000u128),
            storage: BTreeMap::new(),
            code: vec![],
        },
    );
    log::info!("Adding balance for {}", gen_evm_account_id);

    GenesisConfig {
        frame_system: Some(SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_indices: Some(Default::default()),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts.clone(),
        }),
        pallet_validator_set: Some(ValidatorSetConfig {
            validators: initial_authorities
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<_>>(),
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.1.clone(),
                        session_keys(x.2.clone(), x.3.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        pallet_aura: None,
        pallet_grandpa: None,
        pallet_sudo: Some(SudoConfig {
            // Assign network admin rights.
            key: root_key.clone(),
        }),
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                enable_println, // this should only be enabled on development chains
                ..Default::default()
            },
        }),
        pallet_vesting: Some(VestingConfig { vesting: vec![] }),
        pallet_collective_Instance0: Some(Default::default()),
        pallet_collective_Instance1: Some(Default::default()),
        pallet_membership_Instance0: Some(CouncilMembershipConfig {
            members: council_members,
            phantom: Default::default(),
        }),
        pallet_membership_Instance1: Some(TechnicalMembershipConfig {
            members: technical_members,
            phantom: Default::default(),
        }),
        orml_tokens: Some(UniTokensConfig {
            endowed_accounts: endowed_accounts
                .clone()
                .iter()
                .flat_map(|x| vec![(x.0.clone(), CurrencyId::UINK, 10u128.pow(16))])
                .collect(),
        }),
        pallet_treasury: Some(Default::default()),
        pallet_bridge: Some(BridgeConfig {
            validator_accounts: bridge_validators,
            validators_count: 3u32,
            current_limits: vec![
                100 * 10u128.pow(18),
                200 * 10u128.pow(18),
                50 * 10u128.pow(18),
                400 * 10u128.pow(18),
                1 * 10u128.pow(6),
            ],
        }),
        pallet_evm: Some(EVMConfig {
            accounts: evm_accounts,
        }),
        pallet_ethereum: Some(EthereumConfig {}),
    }
}
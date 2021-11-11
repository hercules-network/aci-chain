//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.
// --- aci ---
pub mod chain_spec;
pub mod client;
pub mod service;

pub use service::hadron;
pub use service::quark;

pub use chain_spec::hadron::HadronChainSpec;
pub use chain_spec::quark::QuarkChainSpec;
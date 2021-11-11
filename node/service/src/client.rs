//! Aci Client meta trait

/// aci client abstraction, this super trait only pulls in functionality required for
/// aci internal crates.
// --- crates ---
pub use codec::Codec;
use aci_primitives::{AccountId, AuraId, Balance, BlockNumber, Nonce, OpaqueBlock as Block};
use sp_runtime::traits::BlakeTwo256;
// use sp_consensus::BlockStatus;
// use aci_primitives::{Hash, Header};
// use sc_client_api::{Backend as BackendT, BlockchainEvents, KeyIterator};
// use sp_api::{CallApiAt, NumberFor, ProvideRuntimeApi};
// use sp_blockchain::HeaderBackend;
// use sp_runtime::{
//     generic::{BlockId, SignedBlock},
//     traits::{BlakeTwo256, Block as BlockT},
//     Justification,
// };
// use sp_storage::{ChildInfo, PrefixedStorageKey, StorageData, StorageKey};
// use std::sync::Arc;

/// A set of APIs that Aci-like runtimes must implement.
pub trait RuntimeApiCollection:
    sp_api::ApiExt<Block, Error = sp_blockchain::Error>
    + sp_api::Metadata<Block>
    + sp_block_builder::BlockBuilder<Block>
    + sp_consensus_aura::AuraApi<Block, AuraId>
    + sp_finality_grandpa::GrandpaApi<Block>
    + sp_offchain::OffchainWorkerApi<Block>
    + sp_session::SessionKeys<Block>
    + sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
    + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
    + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
    + pallet_staking_rpc::StakingRuntimeApi<Block, AccountId, Balance>
    + pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>
where
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> RuntimeApiCollection for Api
where
    Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
        + sp_api::ApiExt<Block, Error = sp_blockchain::Error>
        + sp_api::Metadata<Block>
        + sp_block_builder::BlockBuilder<Block>
        + sp_consensus_aura::AuraApi<Block, AuraId>
        + sp_finality_grandpa::GrandpaApi<Block>
        + sp_offchain::OffchainWorkerApi<Block>
        + sp_session::SessionKeys<Block>
        + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
        + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
        + pallet_staking_rpc::StakingRuntimeApi<Block, AccountId, Balance>
        + pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>,
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

/// A set of APIs that Aci-like runtimes must implement.
pub trait RuntimeEvmApiCollection:
    sp_api::ApiExt<Block, Error = sp_blockchain::Error>
    + sp_api::Metadata<Block>
    + sp_block_builder::BlockBuilder<Block>
    + sp_consensus_aura::AuraApi<Block, AuraId>
    + sp_finality_grandpa::GrandpaApi<Block>
    + sp_offchain::OffchainWorkerApi<Block>
    + sp_session::SessionKeys<Block>
    + ap_rpc::EthereumRuntimeRPCApi<Block>
    + sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
    + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
    + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
    + pallet_staking_rpc::StakingRuntimeApi<Block, AccountId, Balance>
    + pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>
where
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> RuntimeEvmApiCollection for Api
where
    Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
        + sp_api::ApiExt<Block, Error = sp_blockchain::Error>
        + sp_api::Metadata<Block>
        + sp_block_builder::BlockBuilder<Block>
        + sp_consensus_aura::AuraApi<Block, AuraId>
        + sp_finality_grandpa::GrandpaApi<Block>
        + sp_offchain::OffchainWorkerApi<Block>
        + sp_session::SessionKeys<Block>
        + ap_rpc::EthereumRuntimeRPCApi<Block>
        + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
        + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
        + pallet_staking_rpc::StakingRuntimeApi<Block, AccountId, Balance>
        + pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>,
    <Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

pub trait AciClient<Block, Backend, Runtime>:
    Sized
    + Send
    + Sync
    + sc_client_api::BlockchainEvents<Block>
    + sp_api::CallApiAt<Block, Error = sp_blockchain::Error, StateBackend = Backend::State>
    + sp_api::ProvideRuntimeApi<Block, Api = Runtime::RuntimeApi>
    + sp_blockchain::HeaderBackend<Block>
where
    Backend: sc_client_api::Backend<Block>,
    Block: sp_runtime::traits::Block,
    Runtime: sp_api::ConstructRuntimeApi<Block, Self>,
{
}

impl<Block, Backend, Runtime, Client> AciClient<Block, Backend, Runtime> for Client
where
    Backend: sc_client_api::Backend<Block>,
    Block: sp_runtime::traits::Block,
    Client: Sized
        + Send
        + Sync
        + sp_api::CallApiAt<Block, Error = sp_blockchain::Error, StateBackend = Backend::State>
        + sp_api::ProvideRuntimeApi<Block, Api = Runtime::RuntimeApi>
        + sp_blockchain::HeaderBackend<Block>
        + sc_client_api::BlockchainEvents<Block>,
    Runtime: sp_api::ConstructRuntimeApi<Block, Self>,
{
}

// /// Config that abstracts over all available client implementations.
// ///
// /// For a concrete type there exists [`Client`].
// pub trait AbstractClient<Block, Backend>:
//         BlockchainEvents<Block>
//         + Sized
//         + Send
//         + Sync
//         + ProvideRuntimeApi<Block>
//         + HeaderBackend<Block>
//         + CallApiAt<Block, StateBackend = Backend::State>
//     where
//         Block: BlockT,
//         Backend: BackendT<Block>,
//         Backend::State: sp_api::StateBackend<BlakeTwo256>,
//         Self::Api: RuntimeApiCollection<StateBackend = Backend::State>,
// {
// }
//
// impl<Block, Backend, Client> AbstractClient<Block, Backend> for Client
//     where
//         Block: BlockT,
//         Backend: BackendT<Block>,
//         Backend::State: sp_api::StateBackend<BlakeTwo256>,
//         Client: BlockchainEvents<Block>
//         + ProvideRuntimeApi<Block>
//         + HeaderBackend<Block>
//         + Sized
//         + Send
//         + Sync
//         + CallApiAt<Block, StateBackend = Backend::State>,
//         Client::Api: RuntimeApiCollection<StateBackend = Backend::State>,
// {
// }
//
// /// Execute something with the client instance.
// ///
// /// As there exist multiple chains inside Moonbeam, like Moonbeam itself, Moonbase,
// /// Moonriver etc, there can exist different kinds of client types. As these
// /// client types differ in the generics that are being used, we can not easily
// /// return them from a function. For returning them from a function there exists
// /// [`Client`]. However, the problem on how to use this client instance still
// /// exists. This trait "solves" it in a dirty way. It requires a type to
// /// implement this trait and than the [`execute_with_client`](ExecuteWithClient:
// /// :execute_with_client) function can be called with any possible client
// /// instance.
// ///
// /// In a perfect world, we could make a closure work in this way.
// pub trait ExecuteWithClient {
//     /// The return type when calling this instance.
//     type Output;
//
//     /// Execute whatever should be executed with the given client instance.
//     fn execute_with_client<Client, Api, Backend>(self, client: Arc<Client>) -> Self::Output
//         where
//             <Api as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
//             Backend: sc_client_api::Backend<Block>,
//             Backend::State: sp_api::StateBackend<BlakeTwo256>,
//             Api: crate::client::RuntimeApiCollection<StateBackend = Backend::State>,
//             Client: AbstractClient<Block, Backend, Api = Api> + 'static;
// }
//
// /// A handle to a Moonbeam client instance.
// ///
// /// The Moonbeam service supports multiple different runtimes (Moonbase, Moonbeam
// /// itself, etc). As each runtime has a specialized client, we need to hide them
// /// behind a trait. This is this trait.
// ///
// /// When wanting to work with the inner client, you need to use `execute_with`.
// pub trait ClientHandle {
//     /// Execute the given something with the client.
//     fn execute_with<T: ExecuteWithClient>(&self, t: T) -> T::Output;
// }
//
// /// A client instance of Moonbeam.
// #[derive(Clone)]
// pub enum Client {
//     Hadron(Arc<crate::service::FullClient<hadron_runtime::RuntimeApi, crate::service::hadron::HadronExecutor>>),
//     Quark(Arc<crate::service::FullClient<quark_runtime::RuntimeApi, crate::service::quark::QuarkExecutor>>),
// }
//
// impl ClientHandle for Client {
//     fn execute_with<T: ExecuteWithClient>(&self, t: T) -> T::Output {
//         match self {
//             Self::Hadron(client) => {
//                 T::execute_with_client::<_, _, crate::service::FullBackend>(t, client.clone())
//             }
//             Self::Quark(client) => {
//                 T::execute_with_client::<_, _, crate::service::FullBackend>(t, client.clone())
//             }
//         }
//     }
// }
//
// impl sc_client_api::UsageProvider<Block> for Client {
//     fn usage_info(&self) -> sc_client_api::ClientInfo<Block> {
//         match self {
//             Self::Hadron(client) => client.usage_info(),
//             Self::Quark(client) => client.usage_info(),
//         }
//     }
// }
//
// impl sc_client_api::BlockBackend<Block> for Client {
//     fn block_body(&self, id: &BlockId<Block>) -> sp_blockchain::Result<Option<Vec<<Block as BlockT>::Extrinsic>>> {
//         match self {
//             Self::Hadron(client) => client.block_body(id),
//             Self::Quark(client) => client.block_body(id),
//         }
//     }
//
//     fn block(&self, id: &BlockId<Block>) -> sp_blockchain::Result<Option<SignedBlock<Block>>> {
//         match self {
//             Self::Hadron(client) => client.block(id),
//             Self::Quark(client) => client.block(id),
//         }
//     }
//
//     fn block_status(&self, id: &BlockId<Block>) -> sp_blockchain::Result<BlockStatus> {
//         match self {
//             Self::Hadron(client) => client.block_status(id),
//             Self::Quark(client) => client.block_status(id),
//         }
//     }
//
//     fn justification(&self, id: &BlockId<Block>) -> sp_blockchain::Result<Option<Justification>> {
//         match self {
//             Self::Hadron(client) => client.justification(id),
//             Self::Quark(client) => client.justification(id),
//         }
//     }
//
//     fn block_hash(&self, number: NumberFor<Block>) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
//         match self {
//             Self::Hadron(client) => client.block_hash(number),
//             Self::Quark(client) => client.block_hash(number),
//         }
//     }
//
//     fn extrinsic(&self, hash: &<Block as BlockT>::Hash) -> sp_blockchain::Result<Option<<Block as BlockT>::Extrinsic>> {
//         match self {
//             Self::Hadron(client) => client.extrinsic(hash),
//             Self::Quark(client) => client.extrinsic(hash),
//         }
//     }
//
//     fn have_extrinsic(&self, hash: &<Block as BlockT>::Hash) -> sp_blockchain::Result<bool> {
//         match self {
//             Self::Hadron(client) => client.have_extrinsic(hash),
//             Self::Quark(client) => client.have_extrinsic(hash),
//         }
//     }
// }
//
// impl sc_client_api::StorageProvider<Block, crate::service::FullBackend> for Client {
//     fn storage(&self, id: &BlockId<Block>, key: &StorageKey) -> sp_blockchain::Result<Option<StorageData>> {
//         match self {
//             Self::Hadron(client) => client.storage(id, key),
//             Self::Quark(client) => client.storage(id, key),
//         }
//     }
//
//     fn storage_keys(&self, id: &BlockId<Block>, key_prefix: &StorageKey) -> sp_blockchain::Result<Vec<StorageKey>> {
//         match self {
//             Self::Hadron(client) => client.storage_keys(id, key_prefix),
//             Self::Quark(client) => client.storage_keys(id, key_prefix),
//         }
//     }
//
//     fn storage_hash(
//         &self,
//         id: &BlockId<Block>,
//         key: &StorageKey,
//     ) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
//         match self {
//             Self::Hadron(client) => client.storage_hash(id, key),
//             Self::Quark(client) => client.storage_hash(id, key),
//         }
//     }
//
//     fn storage_pairs(
//         &self,
//         id: &BlockId<Block>,
//         key_prefix: &StorageKey,
//     ) -> sp_blockchain::Result<Vec<(StorageKey, StorageData)>> {
//         match self {
//             Self::Hadron(client) => client.storage_pairs(id, key_prefix),
//             Self::Quark(client) => client.storage_pairs(id, key_prefix),
//         }
//     }
//
//     fn storage_keys_iter<'a>(
//         &self,
//         id: &BlockId<Block>,
//         prefix: Option<&'a StorageKey>,
//         start_key: Option<&StorageKey>,
//     ) -> sp_blockchain::Result<KeyIterator<'a, <crate::service::FullBackend as sc_client_api::Backend<Block>>::State, Block>> {
//         match self {
//             Self::Hadron(client) => client.storage_keys_iter(id, prefix, start_key),
//             Self::Quark(client) => client.storage_keys_iter(id, prefix, start_key),
//         }
//     }
//
//     fn child_storage(
//         &self,
//         id: &BlockId<Block>,
//         child_info: &ChildInfo,
//         key: &StorageKey,
//     ) -> sp_blockchain::Result<Option<StorageData>> {
//         match self {
//             Self::Hadron(client) => client.child_storage(id, child_info, key),
//             Self::Quark(client) => client.child_storage(id, child_info, key),
//         }
//     }
//
//     fn child_storage_keys(
//         &self,
//         id: &BlockId<Block>,
//         child_info: &ChildInfo,
//         key_prefix: &StorageKey,
//     ) -> sp_blockchain::Result<Vec<StorageKey>> {
//         match self {
//             Self::Hadron(client) => client.child_storage_keys(id, child_info, key_prefix),
//             Self::Quark(client) => client.child_storage_keys(id, child_info, key_prefix),
//         }
//     }
//
//     fn child_storage_hash(
//         &self,
//         id: &BlockId<Block>,
//         child_info: &ChildInfo,
//         key: &StorageKey,
//     ) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
//         match self {
//             Self::Hadron(client) => client.child_storage_hash(id, child_info, key),
//             Self::Quark(client) => client.child_storage_hash(id, child_info, key),
//         }
//     }
//
//     fn max_key_changes_range(
//         &self,
//         first: NumberFor<Block>,
//         last: BlockId<Block>,
//     ) -> sp_blockchain::Result<Option<(NumberFor<Block>, BlockId<Block>)>> {
//         match self {
//             Self::Hadron(client) => client.max_key_changes_range(first, last),
//             Self::Quark(client) => client.max_key_changes_range(first, last),
//         }
//     }
//
//     fn key_changes(
//         &self,
//         first: NumberFor<Block>,
//         last: BlockId<Block>,
//         storage_key: Option<&PrefixedStorageKey>,
//         key: &StorageKey,
//     ) -> sp_blockchain::Result<Vec<(NumberFor<Block>, u32)>> {
//         match self {
//             Self::Hadron(client) => client.key_changes(first, last, storage_key, key),
//             Self::Quark(client) => client.key_changes(first, last, storage_key, key),
//         }
//     }
// }
//
// impl sp_blockchain::HeaderBackend<Block> for Client {
//     fn header(&self, id: BlockId<Block>) -> sp_blockchain::Result<Option<Header>> {
//         match self {
//             Self::Hadron(client) => client.header(&id),
//             Self::Quark(client) => client.header(&id),
//         }
//     }
//
//     fn info(&self) -> sp_blockchain::Info<Block> {
//         match self {
//             Self::Hadron(client) => client.info(),
//             Self::Quark(client) => client.info(),
//         }
//     }
//
//     fn status(&self, id: BlockId<Block>) -> sp_blockchain::Result<sp_blockchain::BlockStatus> {
//         match self {
//             Self::Hadron(client) => client.status(id),
//             Self::Quark(client) => client.status(id),
//         }
//     }
//
//     fn number(&self, hash: Hash) -> sp_blockchain::Result<Option<BlockNumber>> {
//         match self {
//             Self::Hadron(client) => client.number(hash),
//             Self::Quark(client) => client.number(hash),
//         }
//     }
//
//     fn hash(&self, number: BlockNumber) -> sp_blockchain::Result<Option<Hash>> {
//         match self {
//             Self::Hadron(client) => client.hash(number),
//             Self::Quark(client) => client.hash(number),
//         }
//     }
// }
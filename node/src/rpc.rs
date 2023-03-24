//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]


use jsonrpsee::RpcModule;
use node_template_runtime::{opaque::Block, AccountId, Balance, Index, Hash};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_runtime::traits::{BlakeTwo256, Block as BlockT};
use std::{sync::Arc, collections::BTreeMap};

pub use sc_rpc_api::DenyUnsafe;

use sc_client_api::{
	backend::{AuxStore, Backend, StateBackend, StorageProvider},
	client::BlockchainEvents,
};
use sc_transaction_pool::{ChainApi, Pool};
use sc_network::NetworkService;

use fc_rpc::{
	EthBlockDataCacheTask, EthTask, OverrideHandle, RuntimeApiStorageOverride, SchemaV1Override,
	SchemaV2Override, SchemaV3Override, StorageOverride, EthSigner,
};
use fc_rpc_core::types::{CallRequest, FeeHistoryCache, FilterPool};
use fp_storage::EthereumStorageSchema;

// Runtime

/// Full client dependencies.
// pub struct FullDeps<C, P> {
// 	/// The client instance to use.
// 	pub client: Arc<C>,
// 	/// Transaction pool instance.
// 	pub pool: Arc<P>,
// 	/// Whether to deny unsafe calls
// 	pub deny_unsafe: DenyUnsafe,
// }

pub struct FullDeps<C, P, A: ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// The Node authority flag
	pub is_authority: bool,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// EthFilterApi pool.
	// pub filter_pool: Option<FilterPool>,
	/// The list of optional RPC extensions.
	// pub ethapi_cmd: Vec<EthApiCmd>,
	/// Frontier Backend.
	pub frontier_backend: Arc<fc_db::Backend<Block>>,
	/// Backend.
	// pub backend: Arc<BE>,
	/// Manual seal command sink
	// pub command_sink: Option<futures::channel::mpsc::Sender<EngineCommand<Hash>>>,
	/// Maximum number of logs in a query.
	pub max_past_logs: u32,
	/// Maximum fee history cache size.
	pub fee_history_limit: u64,
	/// Fee history cache.
	pub fee_history_cache: FeeHistoryCache,
	/// Channels for manual xcm messages (downward, hrmp)
	// pub xcm_senders: Option<(flume::Sender<Vec<u8>>, flume::Sender<(ParaId, Vec<u8>)>)>,
	/// Ethereum data access overrides.
	pub overrides: Arc<OverrideHandle<Block>>,
	/// Cache for Ethereum block data.
	pub block_data_cache: Arc<EthBlockDataCacheTask<Block>>,
}

// pub struct Eth<B: BlockT, C, P, CT, BE, H: ExHashT, A: ChainApi, EGA = ()> {
// 	pool: Arc<P>,
// 	graph: Arc<Pool<A>>,
// 	client: Arc<C>,
// 	convert_transaction: Option<CT>,
// 	network: Arc<NetworkService<B, H>>,
// 	is_authority: bool,
// 	signers: Vec<Box<dyn EthSigner>>,
// 	overrides: Arc<OverrideHandle<B>>,
// 	backend: Arc<fc_db::Backend<B>>,
// 	block_data_cache: Arc<EthBlockDataCacheTask<B>>,
// 	fee_history_cache: FeeHistoryCache,
// 	fee_history_cache_limit: FeeHistoryCacheLimit,
// 	/// When using eth_call/eth_estimateGas, the maximum allowed gas limit will be
// 	/// block.gas_limit * execute_gas_limit_multiplier
// 	execute_gas_limit_multiplier: u64,
// 	_marker: PhantomData<(B, BE, EGA)>,
// }

pub fn overrides_handle<C, BE>(client: Arc<C>) -> Arc<OverrideHandle<Block>>
where
	C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError>,
	C: Send + Sync + 'static,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	// C::Api: fp_rpc::ConvertTransactionRuntimeApi<Block>,
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
{
	let mut overrides_map = BTreeMap::new();
	overrides_map.insert(
		EthereumStorageSchema::V1,
		Box::new(SchemaV1Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);
	overrides_map.insert(
		EthereumStorageSchema::V2,
		Box::new(SchemaV2Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);
	overrides_map.insert(
		EthereumStorageSchema::V3,
		Box::new(SchemaV3Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);

	Arc::new(OverrideHandle {
		schemas: overrides_map,
		fallback: Box::new(RuntimeApiStorageOverride::new(client.clone())),
	})
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, A, BE>(
	deps: FullDeps<C, P, A>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	// C: ProvideRuntimeApi<Block>,
	C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	C::Api: fp_rpc::ConvertTransactionRuntimeApi<Block>,
	P: TransactionPool<Block = Block> + 'static,
	A: ChainApi<Block = Block> + 'static,
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
{
	use fc_rpc::{
		Eth, 
		EthApiServer,
		// EthFilter, EthFilterApiServer, EthPubSub, EthPubSubApiServer, Net,
		// NetApiServer, Web3, Web3ApiServer,
	};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	// let FullDeps { client, pool, deny_unsafe } = deps;]

	let FullDeps {
		client,
		pool,
		graph,
		deny_unsafe,
		is_authority,
		network,
		// filter_pool,
		// ethapi_cmd,
		// command_sink,
		frontier_backend,
		// backend: _,
		max_past_logs,
		fee_history_limit,
		fee_history_cache,
		// xcm_senders,
		overrides,
		block_data_cache,
	} = deps;

	module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `module.merge(YourRpcTrait::into_rpc(YourRpcStruct::new(ReferenceToClient, ...)))?;`

	// TODO: are we supporting signing?
	let signers: Vec<Box<dyn EthSigner>> = Vec::new();

	enum Never {}
	impl<T> fp_rpc::ConvertTransaction<T> for Never {
		fn convert_transaction(&self, _transaction: pallet_ethereum::Transaction) -> T {
			// The Never type is not instantiable, but this method requires the type to be
			// instantiated to be called (`&self` parameter), so if the code compiles we have the
			// guarantee that this function will never be called.
			unreachable!()
		}
	}
	let convert_transaction: Option<Never> = None;

	module.merge(
		Eth::new(
			Arc::clone(&client),
			Arc::clone(&pool),
			graph.clone(),
			convert_transaction,
			Arc::clone(&network),
			signers,
			Arc::clone(&overrides),
			Arc::clone(&frontier_backend),
			is_authority,
			Arc::clone(&block_data_cache),
			fee_history_cache,
			fee_history_limit,
			10,
		)
		// .with_estimate_gas_adapter::<MoonbeamEGA>()
		.into_rpc(),
	)?;

	Ok(module)
}

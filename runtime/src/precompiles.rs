use pallet_evm::{Context, Precompile, PrecompileResult, PrecompileSet, PrecompileHandle};
use sp_core::H160;
use sp_std::marker::PhantomData;

use pallet_evm_precompile_collectibles::CollectiblesPrecompile;

pub struct FrontierPrecompiles<R>(PhantomData<R>);

impl<R> FrontierPrecompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
	pub fn used_addresses() -> sp_std::vec::Vec<H160> {
		// sp_std::vec![1, 2, 3, 4, 5, 1024, 1025, 1026]
		sp_std::vec![1026]
			.into_iter()
			.map(|x| hash(x))
			.collect()
	}
}

impl<R> PrecompileSet for FrontierPrecompiles<R>
where
	CollectiblesPrecompile<R>: Precompile,
	R: pallet_evm::Config,
{
	fn execute(
		&self,
		handle: &mut impl PrecompileHandle
		// address: H160,
		// input: &[u8],
		// target_gas: Option<u64>,
		// context: &Context,
		// is_static: bool,
	) -> Option<PrecompileResult> {
		// match address {
		// 	Ethereum precompiles :
		// 	a if a == hash(1) => Some(ECRecover::execute(input, target_gas, context, is_static)),
		// 	a if a == hash(2) => Some(Sha256::execute(input, target_gas, context, is_static)),
		// 	a if a == hash(3) => Some(Ripemd160::execute(input, target_gas, context, is_static)),
		// 	a if a == hash(4) => Some(Identity::execute(input, target_gas, context, is_static)),
		// 	a if a == hash(5) => Some(Modexp::execute(input, target_gas, context, is_static)),
		// 	Non-Frontier specific nor Ethereum precompiles :
		// 	a if a == hash(1024) => {
		// 		Some(Sha3FIPS256::execute(input, target_gas, context, is_static))
		// 	}
		// 	a if a == hash(1025) => Some(ECRecoverPublicKey::execute(
		// 		input, target_gas, context, is_static,
		// 	)),
		// 	_ => None,
		// }
		// None
		match handle.code_address() {
			// Ethereum precompiles :
			// a if a == hash(1) => Some(ECRecover::execute(handle)),
			// a if a == hash(2) => Some(Sha256::execute(handle)),
			// a if a == hash(3) => Some(Ripemd160::execute(handle)),
			// a if a == hash(4) => Some(Identity::execute(handle)),
			// a if a == hash(5) => Some(Modexp::execute(handle)),
			// // Non-Frontier specific nor Ethereum precompiles :
			// a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
			// a if a == hash(1025) => Some(ECRecoverPublicKey::execute(handle)),
			a if a == hash(1026) => Some(CollectiblesPrecompile::execute(handle)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160) -> bool {
		Self::used_addresses().contains(&address)
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

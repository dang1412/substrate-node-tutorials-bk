#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
};
use fp_evm::{
	// Precompile,
	PrecompileHandle,
};

use sp_std::{marker::PhantomData};
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;

pub struct CollectiblesPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl<Runtime> CollectiblesPrecompile<Runtime>
where
	Runtime: pallet_collectibles::Config + pallet_evm::Config,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	Runtime::RuntimeCall: From<pallet_collectibles::Call<Runtime>>,
	// BalanceOf<Runtime>: TryFrom<U256> + Into<U256> + EvmData,
	// H256: From<<Runtime as frame_system::Config>::Hash>
	// 	+ Into<<Runtime as frame_system::Config>::Hash>,
{
// impl Precompile for CollectiblesPrecompile {
	#[precompile::public("create_collectible()")]
    fn create_collectible(handle: &mut impl PrecompileHandle) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let call = pallet_collectibles::Call::<Runtime>::create_collectible {};
		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;
		// let input = handle.input();
		// let mut input_offset = 0;

		Ok(())

		// // Yellowpaper: whenever the input is too short, the missing bytes are
		// // considered to be zero.
		// let mut base_len_buf = [0u8; 32];
		// read_input(input, &mut base_len_buf, &mut input_offset);
		// let mut exp_len_buf = [0u8; 32];
		// read_input(input, &mut exp_len_buf, &mut input_offset);
		// let mut mod_len_buf = [0u8; 32];
		// read_input(input, &mut mod_len_buf, &mut input_offset);
    }
}
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_randomness.
pub trait WeightInfo {
    fn set_babe_randomness_results() -> Weight;
}

/// Weights for pallet_randomness using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Randomness RelayEpoch (r:1 w:1)
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: ParachainSystem RelayStateProof (r:1 w:0)
	// Storage: Randomness RandomnessResults (r:0 w:1)
	// Storage: Randomness InherentIncluded (r:0 w:1)
	fn set_babe_randomness_results() -> Weight {
		// Minimum execution time: 14_410 nanoseconds.
		Weight::from_ref_time(14_737_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Randomness RelayEpoch (r:1 w:1)
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: ParachainSystem RelayStateProof (r:1 w:0)
	// Storage: Randomness RandomnessResults (r:0 w:1)
	// Storage: Randomness InherentIncluded (r:0 w:1)
	fn set_babe_randomness_results() -> Weight {
		// Minimum execution time: 14_410 nanoseconds.
		Weight::from_ref_time(14_737_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}

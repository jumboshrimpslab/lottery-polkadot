#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_lottery.
pub trait WeightInfo {
    fn deposit(x: u32, y: u32, ) -> Weight;
    fn request_withdraw(x: u32, y: u32, ) -> Weight;
    fn claim_my_winnings(y: u32, ) -> Weight;
    fn start_lottery() -> Weight;
    fn stop_lottery() -> Weight;
    fn draw_lottery(x: u32, y: u32, ) -> Weight;
    fn process_matured_withdrawals() -> Weight;
    fn set_min_deposit() -> Weight;
    fn set_min_withdraw() -> Weight;
    fn set_gas_reserve() -> Weight;
}

/// Weights for pallet_lottery using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Lottery MinDeposit (r:1 w:0)
	// Storage: Scheduler Lookup (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking SelectedCandidates (r:1 w:0)
	// Storage: Lottery StakedCollators (r:2 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:7 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: Lottery ActiveBalancePerUser (r:1 w:1)
	// Storage: Lottery TotalPot (r:1 w:1)
	// Storage: Lottery TotalUsers (r:1 w:1)
	// Storage: Lottery SumOfDeposits (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	/// The range of component `x` is `[0, 1000]`.
	/// The range of component `y` is `[0, 63]`.
	fn deposit(x: u32, y: u32, ) -> Weight {
		// Minimum execution time: 183_213 nanoseconds.
		Weight::from_ref_time(191_155_541)
			// Standard Error: 525
			.saturating_add(Weight::from_ref_time(158_552).saturating_mul(x.into()))
			// Standard Error: 8_276
			.saturating_add(Weight::from_ref_time(295_360).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(25))
			.saturating_add(T::DbWeight::get().writes(13))
	}
	// Storage: Lottery MinWithdraw (r:1 w:0)
	// Storage: Scheduler Lookup (r:1 w:0)
	// Storage: Lottery ActiveBalancePerUser (r:1 w:1)
	// Storage: Lottery WithdrawalRequestQueue (r:1 w:1)
	// Storage: Lottery TotalUsers (r:1 w:1)
	// Storage: Lottery TotalPot (r:1 w:1)
	// Storage: Lottery SurplusUnstakingBalance (r:1 w:1)
	// Storage: ParachainStaking SelectedCandidates (r:1 w:0)
	// Storage: Lottery StakedCollators (r:2 w:1)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: Lottery UnstakingCollators (r:1 w:1)
	// Storage: ParachainStaking AwardedPts (r:1 w:0)
	/// The range of component `x` is `[0, 1000]`.
	/// The range of component `y` is `[0, 63]`.
	fn request_withdraw(x: u32, y: u32, ) -> Weight {
		// Minimum execution time: 110_186 nanoseconds.
		Weight::from_ref_time(114_775_116)
			// Standard Error: 1_466
			.saturating_add(Weight::from_ref_time(106_892).saturating_mul(x.into()))
			// Standard Error: 23_106
			.saturating_add(Weight::from_ref_time(106_427).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: Lottery UnclaimedWinningsByAccount (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Lottery SumOfDeposits (r:1 w:0)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:1)
	/// The range of component `y` is `[0, 63]`.
	fn claim_my_winnings(y: u32, ) -> Weight {
		// Minimum execution time: 47_723 nanoseconds.
		Weight::from_ref_time(50_397_562)
			// Standard Error: 1_406
			.saturating_add(Weight::from_ref_time(151_698).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:0)
	// Storage: Lottery GasReserve (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn start_lottery() -> Weight {
		// Minimum execution time: 40_198 nanoseconds.
		Weight::from_ref_time(43_685_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn stop_lottery() -> Weight {
		// Minimum execution time: 28_772 nanoseconds.
		Weight::from_ref_time(29_405_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:1 w:0)
	// Storage: Lottery WithdrawalRequestQueue (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:1)
	// Storage: Lottery GasReserve (r:1 w:0)
	// Storage: Lottery SumOfDeposits (r:1 w:0)
	// Storage: Lottery TotalPot (r:1 w:0)
	// Storage: Lottery ActiveBalancePerUser (r:2 w:0)
	// Storage: Lottery UnclaimedWinningsByAccount (r:1 w:1)
	// Storage: Lottery UnstakingCollators (r:1 w:0)
	/// The range of component `x` is `[0, 1000]`.
	/// The range of component `y` is `[0, 63]`.
	fn draw_lottery(x: u32, y: u32, ) -> Weight {
		// Minimum execution time: 78_612 nanoseconds.
		Weight::from_ref_time(79_396_000)
			// Standard Error: 46_210
			.saturating_add(Weight::from_ref_time(908_503).saturating_mul(x.into()))
			// Standard Error: 735_204
			.saturating_add(Weight::from_ref_time(17_320_055).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(y.into())))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Lottery UnstakingCollators (r:1 w:0)
	// Storage: Lottery WithdrawalRequestQueue (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:0)
	// Storage: Lottery GasReserve (r:1 w:0)
	fn process_matured_withdrawals() -> Weight {
		// Minimum execution time: 52_626 nanoseconds.
		Weight::from_ref_time(53_534_000)
			.saturating_add(T::DbWeight::get().reads(6))
	}
    fn set_min_deposit() -> Weight {
		// Minimum execution time: 52_626 nanoseconds.
		Weight::from_ref_time(53_534_000)
			.saturating_add(T::DbWeight::get().reads(6))
	}
    fn set_min_withdraw() -> Weight {
		// Minimum execution time: 52_626 nanoseconds.
		Weight::from_ref_time(53_534_000)
			.saturating_add(T::DbWeight::get().reads(6))
	}
    fn set_gas_reserve() -> Weight {
		// Minimum execution time: 52_626 nanoseconds.
		Weight::from_ref_time(53_534_000)
			.saturating_add(T::DbWeight::get().reads(6))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Lottery MinDeposit (r:1 w:0)
	// Storage: Scheduler Lookup (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking SelectedCandidates (r:1 w:0)
	// Storage: Lottery StakedCollators (r:2 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:7 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: Lottery ActiveBalancePerUser (r:1 w:1)
	// Storage: Lottery TotalPot (r:1 w:1)
	// Storage: Lottery TotalUsers (r:1 w:1)
	// Storage: Lottery SumOfDeposits (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	/// The range of component `x` is `[0, 1000]`.
	/// The range of component `y` is `[0, 63]`.
	fn deposit(x: u32, y: u32, ) -> Weight {
		// Minimum execution time: 183_213 nanoseconds.
		Weight::from_ref_time(191_155_541)
			// Standard Error: 525
			.saturating_add(Weight::from_ref_time(158_552).saturating_mul(x.into()))
			// Standard Error: 8_276
			.saturating_add(Weight::from_ref_time(295_360).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(25))
			.saturating_add(RocksDbWeight::get().writes(13))
	}
	// Storage: Lottery MinWithdraw (r:1 w:0)
	// Storage: Scheduler Lookup (r:1 w:0)
	// Storage: Lottery ActiveBalancePerUser (r:1 w:1)
	// Storage: Lottery WithdrawalRequestQueue (r:1 w:1)
	// Storage: Lottery TotalUsers (r:1 w:1)
	// Storage: Lottery TotalPot (r:1 w:1)
	// Storage: Lottery SurplusUnstakingBalance (r:1 w:1)
	// Storage: ParachainStaking SelectedCandidates (r:1 w:0)
	// Storage: Lottery StakedCollators (r:2 w:1)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: Lottery UnstakingCollators (r:1 w:1)
	// Storage: ParachainStaking AwardedPts (r:1 w:0)
	/// The range of component `x` is `[0, 1000]`.
	/// The range of component `y` is `[0, 63]`.
	fn request_withdraw(x: u32, y: u32, ) -> Weight {
		// Minimum execution time: 110_186 nanoseconds.
		Weight::from_ref_time(114_775_116)
			// Standard Error: 1_466
			.saturating_add(Weight::from_ref_time(106_892).saturating_mul(x.into()))
			// Standard Error: 23_106
			.saturating_add(Weight::from_ref_time(106_427).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(16))
			.saturating_add(RocksDbWeight::get().writes(9))
	}
	// Storage: Lottery UnclaimedWinningsByAccount (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Lottery SumOfDeposits (r:1 w:0)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:1)
	/// The range of component `y` is `[0, 63]`.
	fn claim_my_winnings(y: u32, ) -> Weight {
		// Minimum execution time: 47_723 nanoseconds.
		Weight::from_ref_time(50_397_562)
			// Standard Error: 1_406
			.saturating_add(Weight::from_ref_time(151_698).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:0)
	// Storage: Lottery GasReserve (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn start_lottery() -> Weight {
		// Minimum execution time: 40_198 nanoseconds.
		Weight::from_ref_time(43_685_000)
			.saturating_add(RocksDbWeight::get().reads(6))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn stop_lottery() -> Weight {
		// Minimum execution time: 28_772 nanoseconds.
		Weight::from_ref_time(29_405_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: System Account (r:1 w:0)
	// Storage: Lottery WithdrawalRequestQueue (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:1)
	// Storage: Lottery GasReserve (r:1 w:0)
	// Storage: Lottery SumOfDeposits (r:1 w:0)
	// Storage: Lottery TotalPot (r:1 w:0)
	// Storage: Lottery ActiveBalancePerUser (r:2 w:0)
	// Storage: Lottery UnclaimedWinningsByAccount (r:1 w:1)
	// Storage: Lottery UnstakingCollators (r:1 w:0)
	/// The range of component `x` is `[0, 1000]`.
	/// The range of component `y` is `[0, 63]`.
	fn draw_lottery(x: u32, y: u32, ) -> Weight {
		// Minimum execution time: 78_612 nanoseconds.
		Weight::from_ref_time(79_396_000)
			// Standard Error: 46_210
			.saturating_add(Weight::from_ref_time(908_503).saturating_mul(x.into()))
			// Standard Error: 735_204
			.saturating_add(Weight::from_ref_time(17_320_055).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(11))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(y.into())))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Lottery UnstakingCollators (r:1 w:0)
	// Storage: Lottery WithdrawalRequestQueue (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: Lottery TotalUnclaimedWinnings (r:1 w:0)
	// Storage: Lottery GasReserve (r:1 w:0)
	fn process_matured_withdrawals() -> Weight {
		// Minimum execution time: 52_626 nanoseconds.
		Weight::from_ref_time(53_534_000)
			.saturating_add(RocksDbWeight::get().reads(6))
	}
    fn set_min_deposit() -> Weight {
		// Minimum execution time: 52_626 nanoseconds.
		Weight::from_ref_time(53_534_000)
			.saturating_add(RocksDbWeight::get().reads(6))
	}
    fn set_min_withdraw() -> Weight {
		// Minimum execution time: 52_626 nanoseconds.
		Weight::from_ref_time(53_534_000)
			.saturating_add(RocksDbWeight::get().reads(6))
	}
    fn set_gas_reserve() -> Weight {
		// Minimum execution time: 52_626 nanoseconds.
		Weight::from_ref_time(53_534_000)
			.saturating_add(RocksDbWeight::get().reads(6))
	}
}

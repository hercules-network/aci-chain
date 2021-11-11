use super::frame_system_config::BlocksPerDay;
use crate::{weights::pallet_rewards::WeightInfo, *};
use aci_primitives::*;
use pallet_rewards::Config;
use pallet_staking;

pub use crate::constants::currency::*;
pub use crate::constants::time::*;

/*
    150_000_000 UARTs will be minted during 6 years.
    8 UARTs/block. Total supply is 250_000_000.
    75 days of mining in the sixth year.

    1: 42048000 -> 16.8%
    2: 33638400 -> 13.4%
    3: 26910720 -> 10.8%
    4: 21528576 -> 8.6%
    5: 17222860 -> 6.9%
    6: 8651443  -> 3.5%
*/
frame_support::parameter_types! {
    pub const MiningRewardPerBlock: Balance = 8 * ACI;
    pub const RewardThreshold: Balance = 30 * (BlocksPerDay::get() as Balance) * MiningRewardPerBlock::get();
    pub const StakingRewardPerBlock: Balance = 1 * ACI;
    pub const AmpFactor: Balance = 1e12 as Balance;
    pub const BlocksPerYear: u32 = 365 * BlocksPerDay::get();
    pub const MiningCap: Balance = 150_000_000 * ACI;
}

pub struct AccountIdOf;
impl<T> Convert<T, Option<T>> for AccountIdOf {
    fn convert(a: T) -> Option<T> {
        Some(a)
    }
}

pub struct ConvertNumberToBalance;
impl<BlockNumber, Balance: Bounded + core::convert::From<BlockNumber>> Convert<BlockNumber, Balance>
    for ConvertNumberToBalance
{
    fn convert(a: BlockNumber) -> Balance {
        Balance::saturated_from::<BlockNumber>(a)
    }
}

impl Config for Runtime {
    type AccountIdOf = AccountIdOf;
    type Balance = Balance;
    type Currency = Aci;
    type RewardThreshold = RewardThreshold;
    type RewardPerBlock = MiningRewardPerBlock;
    type BlocksPerYear = BlocksPerYear;
    type MiningCap = MiningCap;
    type Event = Event;
    type WeightInfo = WeightInfo<Runtime>;
}

impl pallet_staking::Config for Runtime {
    type ModuleId = StakingModuleId;
    type Event = Event;
    type Currency = Aci;
    type RewardPerBlock = StakingRewardPerBlock;
    type Id = u32;
    type AmpFactor = AmpFactor;
    type ConvertNumberToBalance = ConvertNumberToBalance;
    type WeightInfo = ();
}

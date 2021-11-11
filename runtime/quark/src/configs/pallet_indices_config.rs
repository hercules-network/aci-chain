use crate::{weights::pallet_indices::WeightInfo, *};
use aci_primitives::*;
use pallet_indices::Config;

frame_support::parameter_types! {
    pub const IndexDeposit: Balance = 1 * ACI;
}
impl Config for Runtime {
    type AccountIndex = AccountIndex;
    type Currency = Aci;
    type Deposit = IndexDeposit;
    type Event = Event;
    type WeightInfo = WeightInfo<Runtime>;
}

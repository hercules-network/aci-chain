use crate::constants::currency::*;
use crate::*;
use aci_primitives::*;
use pallet_validator_set::Config;
use sp_runtime::Perbill;

frame_support::parameter_types! {
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
    pub const ValidatorMortgageLimit: Balance = 10_000 * ACI;
}

impl Config for Runtime {
    type Event = Event;
    type Currency = Aci;
    type ValidatorMortgageLimit = ValidatorMortgageLimit;
}

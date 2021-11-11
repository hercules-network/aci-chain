use super::pallet_membership_config::EnsureRootOrMoreThanHalfCouncil;
use crate::*;
use aci_primitives::*;
use pallet_identity::Config;

frame_support::parameter_types! {
    pub const BasicDeposit: Balance = 10 * ACI;            // 258 bytes on-chain
    pub const FieldDeposit: Balance = 250 * MICRO;          // 66 bytes on-chain
    pub const SubAccountDeposit: Balance = 2 * ACI;        // 53 bytes on-chain
    pub const MaxSubAccounts: u32 = 100;
    pub const MaxAdditionalFields: u32 = 100;
    pub const MaxRegistrars: u32 = 20;
}
impl Config for Runtime {
    type Event = Event;
    type Currency = Aci;
    type BasicDeposit = BasicDeposit;
    type FieldDeposit = FieldDeposit;
    type SubAccountDeposit = SubAccountDeposit;
    type MaxSubAccounts = MaxSubAccounts;
    type MaxAdditionalFields = MaxAdditionalFields;
    type MaxRegistrars = MaxRegistrars;
    type Slashed = Treasury;
    type ForceOrigin = EnsureRootOrMoreThanHalfCouncil;
    type RegistrarOrigin = EnsureRootOrMoreThanHalfCouncil;
    type WeightInfo = weights::pallet_identity::WeightInfo<Runtime>;
}

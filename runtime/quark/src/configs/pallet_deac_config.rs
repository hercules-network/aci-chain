use crate::*;
use pallet_deac::Config;

/// Used for the module deac in `./deac.rs`
impl Config for Runtime {
    type ModuleId = AciDeacModuleId;
    type Currency = Aci;
    type Event = Event;
    type WeightInfo = ();
}

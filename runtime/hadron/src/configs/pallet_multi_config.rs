use crate::*;
use pallet_multi::Config;

/// Used for the module nfr-multi in `./deac-multi.rs`
impl Config for Runtime {
    type ModuleId = AciDeacModuleId;
    type MultiCurrency = Currencies;
    type Event = Event;
    type WeightInfo = ();
}

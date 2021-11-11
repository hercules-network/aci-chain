use crate::{weights::pallet_utility::WeightInfo, *};
use pallet_utility::Config;

impl Config for Runtime {
    type Event = Event;
    type Call = Call;
    type WeightInfo = WeightInfo<Runtime>;
}

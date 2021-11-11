use crate::*;
use pallet_sudo::Config;

impl Config for Runtime {
    type Event = Event;
    type Call = Call;
}

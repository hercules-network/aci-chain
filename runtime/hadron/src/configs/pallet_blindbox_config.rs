use crate::*;
use pallet_blindbox::Config;

impl Config for Runtime {
    type LockModuleId = AciBlindBoxModuleId;
    type DeacHandler = Deac;
    type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
    type WeightInfo = ();
}

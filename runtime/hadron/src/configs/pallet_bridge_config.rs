use crate::*;
use aci_primitives::*;
use pallet_bridge::Config;

frame_support::parameter_types! {
    pub const GetBridgeCurrencyId: CurrencyId = CurrencyId::Token(TokenSymbol::USDT);
}

impl Config for Runtime {
    type Event = Event;
    type Currency = Currencies;
    type GetBridgeCurrencyId = GetBridgeCurrencyId;
}

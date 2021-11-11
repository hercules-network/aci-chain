use crate::*;
use aci_primitives::*;
use orml_currencies::Config;

frame_support::parameter_types! {
    pub const GetNativeCurrencyId: CurrencyId = CurrencyId::Native;
}

impl Config for Runtime {
    type Event = Event;
    type MultiCurrency = UniTokens;
    type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type WeightInfo = ();
}

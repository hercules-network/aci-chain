use frame_support::traits::{Currency, OnUnbalanced};
use crate::*;

/// Logic for the author to get a portion of fees.
pub struct Author<R>(sp_std::marker::PhantomData<R>);

impl<R> OnUnbalanced<NegativeImbalance<R>> for Author<R>
    where
        R: pallet_balances::Config + pallet_authorship::Config,
        <R as frame_system::Config>::AccountId: From<aci_primitives::AccountId>,
        <R as frame_system::Config>::AccountId: Into<aci_primitives::AccountId>,
{
    fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
        <pallet_balances::Module<R>>::resolve_creating(&<pallet_authorship::Module<R>>::author(), amount);
    }
}
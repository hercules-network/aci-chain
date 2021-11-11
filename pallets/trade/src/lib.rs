#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit="256"]
use codec::{Decode, Encode};
pub use frame_support::{
    construct_runtime, decl_event, decl_module, decl_storage, decl_error,
    dispatch::DispatchResult,
    ensure, parameter_types, Parameter,
    traits::{
        Currency, LockableCurrency, ExistenceRequirement, Get, Imbalance, KeyOwnerProofSystem, OnUnbalanced,
        Randomness, WithdrawReasons
    },
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        DispatchInfo, GetDispatchInfo, IdentityFee, Pays, PostDispatchInfo, Weight,
        WeightToFeePolynomial,
    }, StorageValue, debug,
};

use frame_system::{self as system, ensure_signed};
use sp_runtime::sp_std::prelude::Vec;
use sp_runtime::{
    ModuleId, SaturatedConversion,
    traits::{AccountIdConversion}, RuntimeDebug,
};
use sp_std::prelude::*;
use aci_support::DeacManager;
use aci_primitives::CurrencyId;
use orml_traits::MultiCurrency;
use pallet_multi as pallet_deac;

mod default_weight;

pub trait WeightInfo {
    fn create_sale_order() -> Weight;
    fn cancel_sale_order() -> Weight;
    fn accept_sale_order() -> Weight;
    fn create_separable_sale_order() -> Weight;
    fn cancel_separable_sale_order() -> Weight;
    fn accept_separable_sale_order() -> Weight;
}

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
pub struct SaleOrder<AccountId> {
    pub order_id: u64,
    pub collection_id: u64,
    pub item_id: u64,
    pub currency_id: CurrencyId,
    pub value: u64,
    pub owner: AccountId,
    pub price: u64, // maker order's price\
}

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
pub struct SplitSaleOrder<AccountId> {
    pub order_id: u64,
    pub collection_id: u64,
    pub item_id: u64,
    pub currency_id: CurrencyId,
    pub value: u64,
    pub balance: u64,
    pub owner: AccountId,
    pub price: u64, // maker order's price\
}

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
pub struct SaleOrderHistory<AccountId, BlockNumber> {
    pub collection_id: u64,
    pub item_id: u64,
    pub currency_id: CurrencyId,
    pub value: u64,
    pub seller: AccountId,
    pub buyer: AccountId,
    pub price: u64,
    pub buy_time: BlockNumber,
}

pub trait Config: system::Config + pallet_deac::Config {
    /// The DEAC's module id, used for deriving its sovereign account ID.
    type LockModuleId: Get<ModuleId>;

    /// Deac manager.
    type DeacHandler: DeacManager<Self::AccountId, Self::BlockNumber>;

    type Event: From<Event<Self>> + Into<<Self as system::Config>::Event>;

    /// Weight information for the extrinsics in this module.
    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Config> as DeacTrade {

        /// Consignment
        pub SaleOrderList get(fn deac_trade_id): double_map hasher(blake2_128_concat) u64, hasher(blake2_128_concat) u64 => SaleOrder<T::AccountId>;

        /// Consignment SaleOrder by order_id
        pub SaleOrderByIdList get(fn sale_order_id): map hasher(identity) u64 => SaleOrder<T::AccountId>;

        /// Separable SaleOrder
        pub SeparableSaleOrder get(fn separablet_order_id): map hasher(identity) u64 => SplitSaleOrder<T::AccountId>;

        /// Separable SaleOrder List
        pub SeparableSaleOrderList get(fn separablet_order_list_id):double_map hasher(blake2_128_concat) u64, hasher(blake2_128_concat) u64 => Vec<u64>;

        /// Sales history
        pub HistorySaleOrderList get(fn deac_trade_history_id): double_map hasher(blake2_128_concat) u64, hasher(blake2_128_concat) u64 => Vec<SaleOrderHistory<T::AccountId, T::BlockNumber>>;

        /// Next Order id
        pub NextOrderID: u64 = 1;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        CurrencyId = CurrencyId,
    {
        ItemOrderCreated(u64, u64, u64, u64, AccountId, u64, CurrencyId),
        ItemOrderCancel(u64, u64, u64),
        ItemOrderSucceed(u64, u64, AccountId, AccountId, u64, u64, u64, CurrencyId),
        ItemSeparableOrderCreated(u64, u64, u64, u64, u64, AccountId, CurrencyId),
        ItemSeparableOrderCancel(u64, u64, u64),
        ItemSeparableOrderSucceed(u64, u64, u64, u64, AccountId, AccountId, u64, CurrencyId),
    }
);

decl_error! {
	pub enum Error for Module<T: Config> {
		SaleOrderNotExists,
        AccountNotDeacOwner,
	}
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

        /// The DEAC's module id, used for deriving its sovereign account ID.
		const ModuleId: ModuleId = T::LockModuleId::get();

        fn deposit_event() = default;

        #[weight = <T as Config>::WeightInfo::create_sale_order()]
        pub fn create_sale_order(origin, collection_id: u64, item_id: u64, value: u64, currency_id: CurrencyId, price: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let item_owner = T::DeacHandler::is_item_owner(sender.clone(), collection_id, item_id);
            ensure!(item_owner, Error::<T>::AccountNotDeacOwner);

            let target_collection = pallet_deac::Module::<T>::collection(collection_id);
            let recipient = Self::deac_account_id();
            let mut card_value: u64 = value;

            if let pallet_deac::CollectionMode::DEAC(_) = target_collection.mode {
                card_value = 1;
            };

            match target_collection.mode
            {
                pallet_deac::CollectionMode::DEAC(_) => T::DeacHandler::transfer_deac(collection_id, item_id, sender.clone(), recipient)?,
                pallet_deac::CollectionMode::Fungible(_)  => T::DeacHandler::transfer_fungible(collection_id, item_id, card_value, sender.clone(), recipient)?,
                pallet_deac::CollectionMode::ReFungible(_, _)  => T::DeacHandler::transfer_refungible(collection_id, item_id, card_value, sender.clone(), recipient)?,
                _ => ()
            };

            let order_id = NextOrderID::get();

            // Create order
            let order = SaleOrder {
                order_id: order_id,
                collection_id: collection_id,
                item_id: item_id,
                currency_id: currency_id,
                value: card_value,
                owner: sender.clone(),
                price: price,
            };

            NextOrderID::mutate(|id| *id += 1);
            <SaleOrderList<T>>::insert(collection_id, item_id, order.clone());
            <SaleOrderByIdList<T>>::insert(order_id, order);

            // call event
            Self::deposit_event(RawEvent::ItemOrderCreated(collection_id, item_id, card_value, price, sender, order_id, currency_id));
            Ok(())
        }

        #[weight = <T as Config>::WeightInfo::cancel_sale_order()]
        pub fn cancel_sale_order(origin, collection_id: u64, item_id: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let target_sale_order = <SaleOrderList<T>>::get(collection_id, item_id);
            let order_id = target_sale_order.order_id;

            let order_owner = Self::is_sale_order_owner(sender.clone(), collection_id, item_id);
            if !order_owner
            {
                let mes = "Account is not sale order owner";
                panic!("{}", mes);
            }

            let target_collection = pallet_deac::Module::<T>::collection(collection_id);
            let locker = Self::deac_account_id();

            match target_collection.mode
            {
                pallet_deac::CollectionMode::DEAC(_) => T::DeacHandler::transfer_deac(collection_id, item_id, locker, sender.clone())?,
                pallet_deac::CollectionMode::Fungible(_)  => T::DeacHandler::transfer_fungible(collection_id, item_id, target_sale_order.value, locker, sender.clone())?,
                pallet_deac::CollectionMode::ReFungible(_, _)  => T::DeacHandler::transfer_refungible(collection_id, item_id, target_sale_order.value, locker, sender.clone())?,
                _ => ()
            };

            <SaleOrderList<T>>::remove(collection_id, item_id);
            <SaleOrderByIdList<T>>::remove(order_id);

            // call event
            Self::deposit_event(RawEvent::ItemOrderCancel(collection_id, item_id, order_id));
            Ok(())
        }

        #[weight = <T as Config>::WeightInfo::accept_sale_order()]
        pub fn accept_sale_order(origin, collection_id: u64, item_id: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(<SaleOrderList<T>>::contains_key(collection_id, item_id), Error::<T>::SaleOrderNotExists);

            let target_sale_order = <SaleOrderList<T>>::get(collection_id, item_id);
            let deac_owner = target_sale_order.owner;
            let price = target_sale_order.price;
            let order_id = target_sale_order.order_id;
            let currency_id = target_sale_order.currency_id;
            let buy_time = <system::Module<T>>::block_number();

            let target_collection = pallet_deac::Module::<T>::collection(collection_id);
            let locker = Self::deac_account_id();

            T::DeacHandler::charge_royalty(sender.clone(), collection_id, item_id, currency_id, price, buy_time)?;

            <T as pallet_deac::Config>::MultiCurrency::transfer(currency_id, &sender, &deac_owner, price.saturated_into())?;

            // Moves deac-multi from locker account into the buyer's account
            match target_collection.mode
            {
                pallet_deac::CollectionMode::DEAC(_) => T::DeacHandler::transfer_deac(collection_id, item_id, locker, sender.clone())?,
                pallet_deac::CollectionMode::Fungible(_)  => T::DeacHandler::transfer_fungible(collection_id, item_id, target_sale_order.value, locker, sender.clone())?,
                pallet_deac::CollectionMode::ReFungible(_, _)  => T::DeacHandler::transfer_refungible(collection_id, item_id, target_sale_order.value, locker, sender.clone())?,
                _ => ()
            };

            // Create order history
            let order_history = SaleOrderHistory {
                collection_id: collection_id,
                item_id: item_id,
                currency_id: currency_id,
                value: target_sale_order.value,
                seller: deac_owner.clone(),
                buyer: sender.clone(),
                price: price,
                buy_time: buy_time,
            };

            let list_exists = <HistorySaleOrderList<T>>::contains_key(collection_id, item_id);
            if list_exists {
                let mut list = <HistorySaleOrderList<T>>::get(collection_id, item_id);
                list.push(order_history);
                <HistorySaleOrderList<T>>::insert(collection_id, item_id, list);
            } else {
                let mut list = Vec::new();
                list.push(order_history);
                <HistorySaleOrderList<T>>::insert(collection_id, item_id, list);
            }

            <SaleOrderList<T>>::remove(collection_id, item_id);
            <SaleOrderByIdList<T>>::remove(order_id);

            // call event
            Self::deposit_event(RawEvent::ItemOrderSucceed(collection_id, item_id, sender, deac_owner.clone(), order_id, target_sale_order.value, price, currency_id));
            Ok(())
        }

        #[weight = <T as Config>::WeightInfo::create_separable_sale_order()]
        pub fn create_separable_sale_order(origin, collection_id: u64, item_id: u64, value: u64, currency_id: CurrencyId, price: u64 ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let item_owner = T::DeacHandler::is_item_owner(sender.clone(), collection_id, item_id);
            ensure!(item_owner, Error::<T>::AccountNotDeacOwner);

            let order_id = NextOrderID::get();

            let target_collection = pallet_deac::Module::<T>::collection(collection_id);

            let recipient = Self::deac_account_id();
            let mut card_value: u64 = value;

            if let pallet_deac::CollectionMode::DEAC(_) = target_collection.mode {
                card_value = 1;
            };

            match target_collection.mode
            {
                pallet_deac::CollectionMode::DEAC(_) => T::DeacHandler::transfer_deac(collection_id, item_id, sender.clone(), recipient)?,
                pallet_deac::CollectionMode::Fungible(_)  => T::DeacHandler::transfer_fungible(collection_id, item_id, card_value, sender.clone(), recipient)?,
                pallet_deac::CollectionMode::ReFungible(_, _)  => T::DeacHandler::transfer_refungible(collection_id, item_id, card_value, sender.clone(), recipient)?,
                _ => ()
            };

            // Create order
            let order = SplitSaleOrder {
                order_id: order_id,
                collection_id: collection_id,
                item_id: item_id,
                currency_id: currency_id,
                value: card_value,
                balance: card_value,
                owner: sender.clone(),
                price: price,
            };

            NextOrderID::mutate(|id| *id += 1);
            <SeparableSaleOrder<T>>::insert(order_id, order);
            let list_exists = <SeparableSaleOrderList>::contains_key(collection_id, item_id);
            if list_exists {
                let mut list = <SeparableSaleOrderList>::get(collection_id, item_id);
                list.push(order_id);
                <SeparableSaleOrderList>::insert(collection_id, item_id, list);
            } else {
                let mut list = Vec::new();
                list.push(order_id);
                <SeparableSaleOrderList>::insert(collection_id, item_id, list);
            }

            // call event
            Self::deposit_event(RawEvent::ItemSeparableOrderCreated(order_id, collection_id, item_id, card_value, price, sender, currency_id));
            Ok(())
        }

        #[weight = <T as Config>::WeightInfo::cancel_separable_sale_order()]
        pub fn cancel_separable_sale_order(origin, order_id: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let target_sale_order = <SeparableSaleOrder<T>>::get(order_id);
            let collection_id = target_sale_order.collection_id;
            let item_id = target_sale_order.item_id;

            let order_owner = Self::is_separable_sale_order_owner(sender.clone(), order_id);
            if !order_owner
            {
                let mes = "Account is not sale order owner";
                panic!("{}", mes);
            }

            let target_collection = pallet_deac::Module::<T>::collection(target_sale_order.collection_id);
            let locker = Self::deac_account_id();

            match target_collection.mode
            {
                pallet_deac::CollectionMode::DEAC(_) => T::DeacHandler::transfer_deac(collection_id, item_id, locker, sender.clone())?,
                pallet_deac::CollectionMode::Fungible(_)  => T::DeacHandler::transfer_fungible(collection_id, item_id, target_sale_order.balance, locker, sender.clone())?,
                pallet_deac::CollectionMode::ReFungible(_, _)  => T::DeacHandler::transfer_refungible(collection_id, item_id, target_sale_order.balance, locker, sender.clone())?,
                _ => ()
            };

            <SeparableSaleOrder<T>>::remove(order_id);
            let list_exists = <SeparableSaleOrderList>::contains_key(collection_id, item_id);
            if list_exists {
                let mut list = <SeparableSaleOrderList>::get(collection_id, item_id);
                let item_contains = list.contains(&order_id.clone());

                if item_contains {
                    list.retain(|&item| item != order_id);
                    <SeparableSaleOrderList>::insert(collection_id, item_id, list);
                }
            }

            // call event
            Self::deposit_event(RawEvent::ItemSeparableOrderCancel(order_id, collection_id, item_id));
            Ok(())
        }

        #[weight = <T as Config>::WeightInfo::accept_sale_order()]
        pub fn accept_separable_sale_order(origin, order_id: u64, value: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(<SeparableSaleOrder<T>>::contains_key(order_id), Error::<T>::SaleOrderNotExists);

            let target_sale_order = <SeparableSaleOrder<T>>::get(order_id);
            let collection_id = target_sale_order.collection_id;
            let item_id = target_sale_order.item_id;
            let deac_owner = target_sale_order.owner;
            let currency_id = target_sale_order.currency_id;
            let price = target_sale_order.price;
            let order_value = target_sale_order.value;
            let balance = target_sale_order.balance;
            let buy_time = <system::Module<T>>::block_number();

            let target_collection = pallet_deac::Module::<T>::collection(collection_id);
            let locker = Self::deac_account_id();

            ensure!(target_sale_order.balance >= value, "Value not enough");
            let remain_value = balance.checked_sub(value).unwrap();
            let checked_value = price.checked_mul(value).unwrap();

            T::DeacHandler::charge_royalty(sender.clone(), collection_id, item_id, currency_id, checked_value, buy_time)?;

            <T as pallet_deac::Config>::MultiCurrency::transfer(currency_id, &sender, &deac_owner, checked_value.into())?;

            // Moves deac-multi from locker account into the buyer's account
            match target_collection.mode
            {
                pallet_deac::CollectionMode::DEAC(_) => T::DeacHandler::transfer_deac(collection_id, item_id, locker, sender.clone())?,
                pallet_deac::CollectionMode::Fungible(_)  => T::DeacHandler::transfer_fungible(collection_id, item_id, value, locker, sender.clone())?,
                pallet_deac::CollectionMode::ReFungible(_, _)  => T::DeacHandler::transfer_refungible(collection_id, item_id, value, locker, sender.clone())?,
                _ => ()
            };

            // Create order history
            let order_history = SaleOrderHistory {
                collection_id: collection_id,
                item_id: item_id,
                currency_id: currency_id,
                value: value,
                seller: deac_owner.clone(),
                buyer: sender.clone(),
                price: price,
                buy_time: buy_time,
            };

            let new_order = SplitSaleOrder {
                order_id: order_id,
                collection_id: collection_id,
                item_id: item_id,
                currency_id: currency_id,
                value: order_value,
                balance: remain_value,
                owner: deac_owner.clone(),
                price: price,
            };

            let list_exists = <HistorySaleOrderList<T>>::contains_key(collection_id, item_id);
            if list_exists {
                let mut list = <HistorySaleOrderList<T>>::get(collection_id, item_id);
                list.push(order_history);
                <HistorySaleOrderList<T>>::insert(collection_id, item_id, list);
            } else {
                let mut list = Vec::new();
                list.push(order_history);
                <HistorySaleOrderList<T>>::insert(collection_id, item_id, list);
            }

            if remain_value == 0 {
                <SeparableSaleOrder<T>>::remove(order_id);
                let list_exists = <SeparableSaleOrderList>::contains_key(collection_id, item_id);
                if list_exists {
                    let mut list = <SeparableSaleOrderList>::get(collection_id, item_id);
                    let item_contains = list.contains(&order_id.clone());
                    if item_contains {
                        list.retain(|&item| item != order_id);
                        <SeparableSaleOrderList>::insert(collection_id, item_id, list);
                    }
                }
            } else {
                let list_exists = <SeparableSaleOrder<T>>::contains_key(order_id);
                if list_exists {
                    <SeparableSaleOrder<T>>::remove(order_id);
                    <SeparableSaleOrder<T>>::insert(order_id, new_order);
                }
            }

            // call event
            Self::deposit_event(RawEvent::ItemSeparableOrderSucceed(order_id, collection_id, item_id, value, sender, deac_owner, price, currency_id));
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    /// The account ID of the DEAC.
	///
	/// This actually does computation. If you need to keep using it, then make sure you cache the
	/// value and only call this once.
    pub fn deac_account_id() -> T::AccountId {
        T::LockModuleId::get().into_account()
    }

    fn is_sale_order_owner(owner: T::AccountId, collection_id: u64, item_id: u64) -> bool {
        let target_sale_order = <SaleOrderList<T>>::get(collection_id, item_id);

        target_sale_order.owner == owner
    }

    fn is_separable_sale_order_owner(owner: T::AccountId, order_id: u64) -> bool {
        let target_sale_order = <SeparableSaleOrder<T>>::get(order_id);

        target_sale_order.owner == owner
    }
}
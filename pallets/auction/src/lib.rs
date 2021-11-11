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
    ModuleId,
    traits::{AccountIdConversion}, RuntimeDebug,
};
use sp_std::prelude::*;
use aci_support::DeacManager;
use aci_primitives::CurrencyId;
use orml_traits::{MultiCurrency, MultiLockableCurrency};
use pallet_multi as pallet_deac;

mod default_weight;

pub trait WeightInfo {
    fn create_auction() -> Weight;
    fn cancel_auction() -> Weight;
    fn bid() -> Weight;
    fn finish_auction() -> Weight;
}

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
pub struct Auction<AccountId, BlockNumber> {
    pub id: u64,
    pub collection_id: u64,
    pub item_id: u64,
    pub currency_id: CurrencyId,
    pub value: u64,
    pub owner: AccountId,
    pub start_price: u64,
    pub increment: u64,
    pub current_price: u64,
    pub start_time: BlockNumber,
    pub end_time: BlockNumber,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
pub struct BidHistory<AccountId, BlockNumber> {
    pub auction_id: u64,
    pub currency_id: CurrencyId,
    pub bidder: AccountId,
    pub bid_price: u64,
    pub bid_time: BlockNumber,
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
    trait Store for Module<T: Config> as DeacAuction {

        /// Next auction id
        pub NextAuctionID: u64 = 1;

        /// Auction
        pub AuctionList get(fn get_auction): double_map hasher(blake2_128_concat) u64, hasher(blake2_128_concat) u64 => Auction<T::AccountId, T::BlockNumber>;

        /// Bid histories
        pub BidHistoryList get(fn bid_history_list): map hasher(identity) u64 => Vec<BidHistory<T::AccountId, T::BlockNumber>>;

        /// Sales history
        pub HistorySaleOrderList get(fn deac_trade_history_id): double_map hasher(blake2_128_concat) u64, hasher(blake2_128_concat) u64 => Vec<SaleOrderHistory<T::AccountId, T::BlockNumber>>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        CurrencyId = CurrencyId,
    {
        AuctionCreated(u64, u64, u64, u64, u64, AccountId, CurrencyId),
        AuctionBid(u64, u64, u64, u64, u64, AccountId, CurrencyId),
        AuctionSucceed(u64, u64, u64, u64, u64, AccountId, AccountId, CurrencyId),
        AuctionCancel(u64, u64, u64),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        DeacInvalidEndTime,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

        /// The Deac's module id, used for deriving its sovereign account ID.
		const ModuleId: ModuleId = T::LockModuleId::get();

        fn deposit_event() = default;

        #[weight = <T as Config>::WeightInfo::create_auction()]
        pub fn create_auction(origin, collection_id: u64, item_id: u64, value: u64, currency_id: CurrencyId, start_price: u64, increment: u64, start_time: T::BlockNumber, end_time: T::BlockNumber) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let now = <system::Module<T>>::block_number();
            ensure!(now < end_time, Error::<T>::DeacInvalidEndTime);

            let auction = Self::get_auction(collection_id, item_id);
            ensure!(auction.id == 0, "The collection is on auction");

            let is_owner = T::DeacHandler::is_item_owner(sender.clone(), collection_id, item_id);
            ensure!(is_owner, "Not Owner");

            let target_collection = pallet_deac::Module::<T>::collection(collection_id);
            let recipient = Self::deac_account_id();

            match target_collection.mode
            {
                pallet_deac::CollectionMode::DEAC(_) => T::DeacHandler::transfer_deac(collection_id, item_id, sender.clone(), recipient)?,
                pallet_deac::CollectionMode::Fungible(_)  => T::DeacHandler::transfer_fungible(collection_id, item_id, value, sender.clone(), recipient)?,
                pallet_deac::CollectionMode::ReFungible(_, _)  => T::DeacHandler::transfer_refungible(collection_id, item_id, value, sender.clone(), recipient)?,
                _ => ()
            };

            // Create auction
            let auction = Auction {
                id: NextAuctionID::get(),
                collection_id: collection_id,
                item_id: item_id,
                currency_id: currency_id,
                value: value,
                owner: sender.clone(),
                start_price: start_price,
                current_price: start_price,
                increment: increment,
                start_time: start_time,
                end_time: end_time,
            };
            let auction_id = auction.id;
            <AuctionList<T>>::insert(collection_id, item_id, auction);

            NextAuctionID::mutate(|id| *id += 1);

            Self::deposit_event(RawEvent::AuctionCreated(auction_id, collection_id, item_id, value, start_price, sender, currency_id));

            Ok(())
        }

        #[weight = <T as Config>::WeightInfo::bid()]
        pub fn bid(origin, collection_id: u64, item_id: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let auction = Self::get_auction(collection_id, item_id);
            ensure!(auction.id > 0, "The collection is not on auction");
            let now = <system::Module<T>>::block_number();
            ensure!(now >= auction.start_time, "Not start");
            ensure!(now <= auction.end_time, "Ended");
            let price = auction.current_price.saturating_add(auction.increment);
            let currency_id = auction.currency_id;
            let free_balance = <T as pallet_deac::Config>::MultiCurrency::free_balance(currency_id, &sender);
            ensure!(free_balance > price.into(), "Insufficient balance");

            let lock_id = Self::auction_lock_id(auction.id);
            <T as pallet_deac::Config>::MultiCurrency::extend_lock(lock_id, currency_id, &sender, price.into())?;


            let bid_history = BidHistory {
                auction_id: auction.id,
                currency_id: currency_id,
                bidder: sender.clone(),
                bid_price: price,
                bid_time: now,
            };

            <BidHistoryList<T>>::mutate(auction.id, |histories| {
                histories.push(bid_history)
            });

            <AuctionList<T>>::mutate(collection_id, item_id, |auction| {
                auction.current_price = price;
            });


            Self::deposit_event(RawEvent::AuctionBid(auction.id, collection_id, item_id, auction.value, price, sender, currency_id));

            Ok(())
        }

        #[weight = <T as Config>::WeightInfo::finish_auction()]
        pub fn finish_auction(origin, collection_id: u64, item_id: u64) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            let auction = Self::get_auction(collection_id, item_id);
            let currency_id = auction.currency_id;
            ensure!(auction.id > 0, "The collection is not on auction");

            let now = <system::Module<T>>::block_number();
            ensure!(now > auction.end_time, "Auction is not over");

            let histories = Self::bid_history_list(auction.id);

            let target_collection = pallet_deac::Module::<T>::collection(collection_id);
            let locker = Self::deac_account_id();

            if let Some(winner) =  histories.last() {
                match target_collection.mode
                {
                    pallet_deac::CollectionMode::DEAC(_) => T::DeacHandler::transfer_deac(collection_id, item_id, locker.clone(), winner.bidder.clone())?,
                    pallet_deac::CollectionMode::Fungible(_)  => T::DeacHandler::transfer_fungible(collection_id, item_id, auction.value, locker.clone(), winner.bidder.clone())?,
                    pallet_deac::CollectionMode::ReFungible(_, _)  => T::DeacHandler::transfer_refungible(collection_id, item_id, auction.value, locker.clone(), winner.bidder.clone())?,
                    _ => ()
                };

                let lock_id = Self::auction_lock_id(auction.id);
                <T as pallet_deac::Config>::MultiCurrency::remove_lock(lock_id, currency_id, &winner.bidder)?;
                <T as pallet_deac::Config>::MultiCurrency::transfer(currency_id, &winner.bidder, &auction.owner, winner.bid_price.into())?;

                for i in 0..(histories.len() - 1) {
                    let h = &histories[i];
                    <T as pallet_deac::Config>::MultiCurrency::remove_lock(lock_id, currency_id, &h.bidder)?;
                }

                // Create order history
                let order_history = SaleOrderHistory {
                    collection_id: collection_id,
                    item_id: item_id,
                    currency_id: currency_id,
                    value: auction.value,
                    seller: auction.owner.clone(),
                    buyer: winner.bidder.clone(),
                    price: winner.bid_price,
                    buy_time: winner.bid_time,
                };
                <HistorySaleOrderList<T>>::mutate(collection_id, item_id, |list|{
                    list.push(order_history);
                });

                T::DeacHandler::charge_royalty(winner.bidder.clone(), collection_id, item_id, currency_id, winner.bid_price, winner.bid_time)?;

                Self::deposit_event(RawEvent::AuctionSucceed(auction.id, collection_id, item_id, auction.value, winner.bid_price, winner.bidder.clone(), auction.owner, currency_id));

            } else {
                // Cancel the auction
                match target_collection.mode
                {
                    pallet_deac::CollectionMode::DEAC(_) => T::DeacHandler::transfer_deac(collection_id, item_id, locker.clone(), auction.owner.clone())?,
                    pallet_deac::CollectionMode::Fungible(_)  => T::DeacHandler::transfer_fungible(collection_id, item_id, auction.value, locker.clone(), auction.owner.clone())?,
                    pallet_deac::CollectionMode::ReFungible(_, _)  => T::DeacHandler::transfer_refungible(collection_id, item_id, auction.value, locker.clone(), auction.owner.clone())?,
                    _ => ()
                };

                Self::deposit_event(RawEvent::AuctionCancel(auction.id, collection_id, item_id));
            }

            <AuctionList<T>>::remove(collection_id, item_id);

            Ok(())
        }

        #[weight = <T as Config>::WeightInfo::cancel_auction()]
        pub fn cancel_auction(origin, collection_id: u64, item_id: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let auction = Self::get_auction(collection_id, item_id);
            ensure!(auction.id > 0, "The collection is not on auction");
            ensure!(auction.owner == sender, "Not owner");
            let histories = Self::bid_history_list(auction.id);
            ensure!(histories.len() == 0, "Already bided");

            let target_collection = pallet_deac::Module::<T>::collection(collection_id);
            let locker = Self::deac_account_id();

            // Moves deac-multi from locker account into the owner's account
            match target_collection.mode {
                pallet_deac::CollectionMode::DEAC(_) => T::DeacHandler::transfer_deac(collection_id, item_id, locker, sender.clone())?,
                pallet_deac::CollectionMode::Fungible(_)  => T::DeacHandler::transfer_fungible(collection_id, item_id, auction.value, locker, sender.clone())?,
                pallet_deac::CollectionMode::ReFungible(_, _)  => T::DeacHandler::transfer_refungible(collection_id, item_id, auction.value, locker, sender.clone())?,
                _ => (),
            };

            <AuctionList<T>>::remove(collection_id, item_id);

            Self::deposit_event(RawEvent::AuctionCancel(auction.id, collection_id, item_id));
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

    fn auction_lock_id(id: u64) -> [u8; 8] {
        let mut lock_id = id.to_be_bytes();
        lock_id[0..3].copy_from_slice(&*b"deac-multi");
        lock_id
    }
}
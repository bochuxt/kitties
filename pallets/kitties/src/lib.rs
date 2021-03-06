#[cfg_attr(not(feature = "std"),no_std)]

pub use pallet::*;



#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{
		sp_runtime::traits::Hash,
		traits::{ Randomness, Currency, tokens::ExistenceRequirement },
		transactional
	};
	use sp_io::hashing::blake2_128;
	use scale_info::TypeInfo;

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[cfg(feature = "std")]
	use serde::{Deserialize,Serialize};


	#[pallet::pallet]
	#[pallet::generate_store(trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type MaxKittyOwned: Get<u32>;

		type KittyRandomness: Randomness<Self::Hash,Self::BlockNumber>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config>{
		Created(T::AccountId, T::Hash),
		PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),
		Transferred(T::AccountId, T::AccountId,T::Hash),
		Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),

	}

	#[pallet::storage]
	#[pallet::getter(fn kitty_cnt)]
	pub(super) type KittyCnt<T:Config> = StorageValue<_, u64,ValueQuery >;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T:Config> = StorageMap<_,Twox64Concat,T::Hash,Kitty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn kitties_owned)]
	pub(super) type KittiesOwned<T:Config> =
	StorageMap<_, Twox64Concat,T::AccountId,BoundedVec<T::Hash,T::MaxKittyOwned>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T:Config>{
		pub kitties: Vec<(T::AccountId, [u8;16],Gender)>,
	}

	#[cfg(feature="std")]
	impl<T:Config> Default for GenesisConfig<T>{
		fn default() -> GenesisConfig<T> {
			GenesisConfig{kitties:vec![]}

		}
	}
	#[pallet::genesis_build]
	impl<T:Config> GenesisBuild<T> for GenesisConfig<T>{
		fn build(&self) {
			for (acct, dna,gender) in &self.kitties{
				let _ = <Pallet<T>>::mint(acct,Some(dna.clone()),Some(gender.clone()));
			}
		}
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config>{
		pub dna:[u8;16],
		pub price: Option<BalanceOf<T>>,
		pub gender: Gender,
		pub owner: AccountOf<T>,
	}

	#[derive(Clone,Encode, Decode, PartialEq, RuntimeDebug, TypeInfo )]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature="std",derive(Serialize, Deserialize))]
	pub enum Gender{
		Male,
		Female,
	}

	#[pallet::error]
	pub enum Error<T> {
		KittyCntOverflow,
		ExceedMaxKittyOwned,
		BuyerIsKittyOwner,
		TransferToSelf,
		KittyNotExist,
		NotKittyOwner,
		KittyNotForSale,
		KittyBidPriceTooLow,
		NotEnoughBalance,

	}

	#[pallet::call]
	impl<T:Config>Pallet<T>{
		// TODO crate kitty
		// TODO set price
		// TODO transfer
		// TODO buy kitty
		// TODO breed kitty
		#[pallet::weight(100)]
		pub fn create_kitty(origin: OriginFor<T>)->DispatchResult{
			let sender= ensure_signed(origin)?;
			let kitty_id = Self::mint(&sender, None, None)?;
			log::info!(" a kitty is born with id:{:?}",kitty_id);
//			Self::deposit_event();
			Ok(())

		}
	}

	// help functions
	impl<T:Config> Pallet<T>{
		fn gen_gender()-> Gender {
			let random=T::KittyRandomness::random(&b"gender"[..]).0;
			match random.as_ref()[0]%2
			{
				0 => Gender::Male,
				_ => Gender::Female,
			}

		}
		fn gen_dna()->[u8;16]{
			let payload = (
				T::KittyRandomness::random(&b"dna"[..]).0,
				<frame_system::Pallet<T>>::block_number(),
				);
			payload.using_encoded(blake2_128)
			//[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
		}

		pub fn breed_dna(kid1: &T::Hash, kid2: &T::Hash) -> Result<[u8; 16], Error<T>> {
			let dna1 = Self::kitties(kid1).ok_or(<Error<T>>::KittyNotExist)?.dna;
			let dna2 = Self::kitties(kid2).ok_or(<Error<T>>::KittyNotExist)?.dna;
			let mut new_dna= Self::gen_dna();
			for i in 0..new_dna.len() {
				new_dna[i] = (new_dna[i] & dna1[i] ) | (!new_dna[i] & dna2[i]);

			}
			// Ok(new_dna)
			Ok(new_dna)

		}

		pub fn mint(
			owner: &T::AccountId,
			dna: Option<[u8;16]>,
			gender: Option<Gender>,
		)-> Result<T::Hash, Error<T>> {

			let kitty = Kitty::<T>{
				dna: dna.unwrap_or_else(Self::gen_dna),
				price:None,
				gender: gender.unwrap_or_else(Self::gen_gender),
				owner:owner.clone(),

			};
			let kitty_id = T::Hashing::hash_of(&kitty);
			let new_cnt= Self::kitty_cnt().checked_add(1)
				.ok_or(<Error<T>>::KittyCntOverflow)?;

			Ok(kitty_id)
		}
	}


}

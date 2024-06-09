//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    pub type Balance = u128;

    #[pallet::storage]
    pub type TotalIssuance<T: Config> = StorageValue<_, Balance,ValueQuery>;

    #[pallet::storage]
    pub type Balances<T: Config> = StorageMap<_, _, T::AccountId, Balance>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn mint_unsafe(
            origin: T::RuntimeOrigin,
            amount: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
			

			if Balances::<T>::contains_key(&who){
				return Err("Cant Mint Again".into());
			}

            Balances::<T>::insert(&who,amount);

			let mut issuance = TotalIssuance::<T>::get();
			issuance += amount;
			TotalIssuance::<T>::put(issuance);

            Ok(())
        }

        pub fn transfer(
            origin: T::RuntimeOrigin,
            dest: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let sender_balance = Balances::<T>::get(&who).ok_or("Not Exists")?;

            if sender_balance < amount {
                return Err("Insfficient Balance".into());
            }

            let reminder = sender_balance
                .checked_sub(amount)
                .ok_or("InsufficientBalance")?;
            Balances::<T>::mutate(dest, |b: &mut Option<u128>| {
                *b = Some(b.unwrap_or(0) + amount)
            });
            Balances::<T>::insert(&who, reminder);

            Ok(())
        }
    }
}

#[cfg(test)]
mod test{

	use frame::testing_prelude::*;
	use super::pallet as currency_pallet;

	construct_runtime! {
		pub struct Runtime{
			System : frame_system,
			Currency: currency_pallet
		}
	}

	#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
	impl frame_system::Config for Runtime {
		type Block = MockBlock<Runtime>;
		// within pallet we just said `<T as frame_system::Config>::AccountId`, now we
		// finally specified it.
		type AccountId = u64;
	}


	impl currency_pallet::Config for Runtime{}


	#[test]
	fn mint_works(){

		TestState::new_empty().execute_with(|| {

			let result = currency_pallet::Pallet::<Runtime>::mint_unsafe(RuntimeOrigin::signed(1),42);

			assert_ok!(result);

			let balance = currency_pallet::Balances::<Runtime>::get(1);

			assert_eq!(balance,Some(42))

		})

	}
}
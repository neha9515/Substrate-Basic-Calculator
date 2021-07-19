#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	pub type num1<T> = StorageValue<_, u32>;
	#[pallet::storage]
	pub type num2<T> = StorageValue<_, u32>;
	
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]

	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Add(u32, T::AccountId, u32, u32),
		Sub(u32, T::AccountId, u32, u32),
		Mul(u32, T::AccountId, u32, u32),
		Div(u32, T::AccountId,u32, u32),
		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn addition(origin: OriginFor<T>, num1 :u32 ,num2 :u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;
			let result = num1 + num2;

			// Update storage.
			<Something<T>>::put(result);
			
			// Emit an event.
			Self::deposit_event(Event::Add(result, who, num1, num2));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn substraction(origin: OriginFor<T>, num1 :u32 ,num2 :u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;
			let result = num1 - num2;

			// Update storage.
			<Something<T>>::put(result);
			
			// Emit an event.
			Self::deposit_event(Event::Sub(result, who, num1, num2));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		
		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn multiplication(origin: OriginFor<T>, num1 :u32 ,num2 :u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;
			let result = num1 * num2;

			// Update storage.
			<Something<T>>::put(result);
			
			// Emit an event.
			Self::deposit_event(Event::Mul(result, who, num1, num2));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn division(origin: OriginFor<T>, num1 :u32 ,num2 :u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;
			let result = num1 / num2;

			// Update storage.
			<Something<T>>::put(result);
			
			// Emit an event.
			Self::deposit_event(Event::Div(result, who, num1, num2));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		
		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}

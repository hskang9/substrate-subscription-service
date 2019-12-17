#![recursion_limit="128"]
#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_module, decl_storage, decl_event, dispatch::Result, ensure, Parameter};
use system::ensure_signed;
use rstd::prelude::*;
use codec::{Encode, Decode};

pub trait Trait: system::Trait + balances::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SubscriptionInfo<Balance: Parameter, AccountId: Parameter, Hash: Parameter, BlockNumber: Parameter> {
	price: Balance,
	to: AccountId,
	blog_id: Hash,
	period: BlockNumber,
}

impl<Balance: Parameter, AccountId: Parameter, Hash: Parameter, BlockNumber: Parameter> SubscriptionInfo<Balance, AccountId, Hash, BlockNumber> {
	pub fn new(
		price: Balance,
		to: AccountId,
		blog_id: Hash,
		period: BlockNumber
	) -> Self {
		SubscriptionInfo { price, to, blog_id, period}
	}

	pub fn set_price(mut self, price: Balance) {
		self.price = price;
	}

	pub fn set_recipient(mut self, recipient: AccountId) {
		self.to = recipient;
	}

	pub fn set_blog_id(mut self, blog_id: Hash) {
		self.blog_id = blog_id;
	}
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		TotalSubscriptions get(total_subscriptions): u64;
		Subscribers get(subscriber): map T::BlockNumber => Vec<(T::AccountId, T::Hash)>;
		SubscriptionOf get(subscription_info): map (T::AccountId, T::Hash) => Option<SubscriptionInfo<T::Balance, T::AccountId, T::Hash, T::BlockNumber>>;
		Subscriptions get(Subscription): map T::Hash => SubscriptionInfo<T::Balance, T::AccountId, T::Hash, T::BlockNumber>;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		fn on_initialize(block_number: T::BlockNumber) {
			let subscribers = Self::call(block_number);
			for i in subscribers {
				if let Err(e) = Self::process_subscription(i.0, i.1) {
					sr_primitives::print(e);
				}
			}
		}

		pub fn subscribe(origin, recipient: T::AccountId, period: T::BlockNumber, blog_id: T::Hash) -> Result {
			let sender = ensure_signed(origin)?;
			// pay initial price
			let subscription_info = Self::subscription_info()
			<balances::Module<T>>::transfer(sender, )
			// set subscription
			let price = Self::to_balance(1, "one");
			let new_subscription = SubscriptionInfo::new(price, recipient, blog_id, period);
			<SubscriptionOf<T>>::insert((sender.clone(), blog_id), new_subscription);
			let now = <system::Module<T>>::block_number();
			if <Calls<T>>::exists(now+period) {
				<Calls<T>>::mutate(now+period, |c: &mut Vec<(T::AccountId, T::Hash)>| c.push((sender.clone(), blog_id)));
			} else {
				<Calls<T>>::insert(now+period, vec![(sender, blog_id)]);
			}
			
			Ok(())
		}

		pub fn process_subscription(origin, subscriber: T::AccountId, blog_id: T::Hash) -> Result {
			
		}
	}
}



decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		SubscriptionPaid(AccountId),
	}
);


impl<T: Trait> Module<T> {

	pub fn process_subscription(account: T::AccountId, blog_id: T::Hash) -> Result {
		
		return Ok(());
	}
	
	pub fn to_balance(u: u32, digit: &str) -> T::Balance {
		let power = |u: u32, p: u32| -> T::Balance {
			let mut base = T::Balance::from(u);
			for _i in 0..p { 
				base *= T::Balance::from(10)
			}
			return base;
		};
		let result = match digit  {
			"femto" => T::Balance::from(u),
			"nano" =>  power(u, 3),
			"micro" => power(u, 6),
			"milli" => power(u, 9),
			"one" => power(u,12),
			"kilo" => power(u, 15),
			"mega" => power(u, 18),
			"giga" => power(u, 21),
			"tera" => power(u, 24),
			"peta" => power(u, 27),
			"exa" => power(u, 30),
			"zetta" => power(u, 33),
			"yotta" => power(u, 36),
			_ => T::Balance::from(u)
		}; 
		result 
	}
}


use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
	balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
	AccountId: Ord + Clone,
	Balance: CheckedAdd + CheckedSub + Zero + Copy,
{
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&Balance::zero())
	}

	pub fn transfer(
		&mut self,
		from: &AccountId,
		to: &AccountId,
		amount: Balance,
	) -> Result<(), &'static str> {
		let from_balance = self.balance(from);
		let to_balance = self.balance(to);

		let new_from_balance = from_balance.checked_sub(&amount).ok_or("Not enough founds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("overflow to_balance")?;

		self.set_balance(from, new_from_balance);
		self.set_balance(to, new_to_balance);
		Ok(())
	}
}

mod tests {

	use std::u128::MAX as MAX_U128;

	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<String, u128>::new();

		assert_eq!(balances.balance(&"Alice".to_string()), 0);
		balances.set_balance(&"Alice".to_string(), 100);
		assert_eq!(balances.balance(&"Alice".to_string()), 100);
		assert_eq!(balances.balance(&"Bob".to_string()), 0);
	}

	#[test]
	fn transfer() {
		let mut balances = super::Pallet::<String, u128>::new();

		assert!(matches!(
			balances.transfer(&"alice".to_string(), &"bob".to_string(), 123),
			Err("Not enough founds.")
		));

		balances.set_balance(&"alice".to_string(), 10000u128);
		assert_eq!(balances.transfer(&"alice".to_string(), &"bob".to_string(), 123), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 9877);
		assert_eq!(balances.balance(&"bob".to_string()), 123);

		assert_eq!(
			balances.transfer(&"alice".to_string(), &"bob".to_string(), 99999),
			Err("Not enough founds.")
		);

		balances.set_balance(&"bob".to_string(), MAX_U128);

		assert!(matches!(
			balances.transfer(&"alice".to_string(), &"bob".to_string(), 123),
			Err("overflow to_balance")
		))
	}
}

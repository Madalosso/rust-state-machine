use std::{collections::BTreeMap, ops::AddAssign};

use num::{one, zero, One, Zero};

pub trait Config {
	type AccountId: Clone + Ord;
	type BlockNumber: Copy + Zero + AddAssign + One;
	type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Pallet { block_number: zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += one();
	}

	pub fn get_nonce(&self, caller: &T::AccountId) -> T::Nonce {
		match self.nonce.get(caller) {
			None => zero(),
			Some(n) => *n,
		}
	}

	pub fn inc_nonce(&mut self, caller: &T::AccountId) {
		let nonce = self.get_nonce(caller);
		// let nonce = *self.nonce.get(caller).unwrap_or(&Nonce::zero());
		let new_nonce = nonce + one();
		self.nonce.insert(caller.clone(), new_nonce);
	}
}

mod test {

	struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let mut system = super::Pallet::<TestConfig>::new();
		assert_eq!(system.block_number(), 0);
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn init_nonce() {
		let mut system = super::Pallet::<TestConfig>::new();
		assert_eq!(system.get_nonce(&"Alice".to_string()), 0);

		system.inc_nonce(&"Alice".to_string());
		assert_eq!(system.nonce.get(&"Alice".to_string()), Some(&1));
		system.inc_nonce(&"Alice".to_string());
		assert_eq!(system.nonce.get(&"Alice".to_string()), Some(&2));
		system.inc_nonce(&"Bob".to_string());
		assert_eq!(system.nonce.get(&"Bob".to_string()), Some(&1));
	}
}

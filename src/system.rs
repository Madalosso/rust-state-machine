use std::{collections::BTreeMap, ops::AddAssign};

use num::{one, zero, One, Zero};

// use crate::types::{AccountId, BlockNumber, Nonce};

#[derive(Debug)]
pub struct Pallet<BlockNumber, AccountId, Nonce> {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}

impl<BlockNumber, AccountId, Nonce> Pallet<BlockNumber, AccountId, Nonce>
where
	BlockNumber: Copy + Zero + AddAssign + One,
	AccountId: Clone + Ord,
	Nonce: Zero + One + Copy,
{
	pub fn new() -> Self {
		Pallet { block_number: zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += one();
	}

	pub fn get_nonce(&self, caller: &AccountId) -> Nonce {
		match self.nonce.get(caller) {
			None => zero(),
			Some(n) => *n,
		}
	}

	pub fn inc_nonce(&mut self, caller: &AccountId) {
		let nonce = self.get_nonce(caller);
		// let nonce = *self.nonce.get(caller).unwrap_or(&Nonce::zero());
		let new_nonce = nonce + one();
		self.nonce.insert(caller.clone(), new_nonce);
	}
}

mod test {

	#[test]
	fn init_system() {
		let mut system = super::Pallet::<u128, String, u32>::new();
		assert_eq!(system.block_number(), 0);
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn init_nonce() {
		let mut system = super::Pallet::<u128, String, u32>::new();
		assert_eq!(system.get_nonce(&"Alice".to_string()), 0);

		system.inc_nonce(&"Alice".to_string());
		assert_eq!(system.nonce.get(&"Alice".to_string()), Some(&1));
		system.inc_nonce(&"Alice".to_string());
		assert_eq!(system.nonce.get(&"Alice".to_string()), Some(&2));
		system.inc_nonce(&"Bob".to_string());
		assert_eq!(system.nonce.get(&"Bob".to_string()), Some(&1));
	}
}

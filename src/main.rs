use support::Dispatch;

mod balances;
mod support;
mod system;
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
	// NOTE: Omit the from field. Caller will always be "derived" from Dispatch::Caller (And will always be types::AccountId)
	BalancesTransfer { to: types::AccountId, amount: types::Balance },
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl support::Dispatch for Runtime {
	type Caller = types::AccountId;
	type Call = RuntimeCall;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::BalancesTransfer { to, amount } => {
				self.balances.transfer(&caller, &to, amount)?;
			},
		}
		Ok(())
	}
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err("Invalid block number");
		}

		for support::Extrinsic { caller, call } in block.extrinsics.into_iter() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| eprintln!("{}", e));
		}

		Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();

	let alice = "Alice".to_string();
	let bob = "Bob".to_string();

	runtime.balances.set_balance(&alice, 100);

	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::BalancesTransfer { to: bob.clone(), amount: 50 },
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::BalancesTransfer { to: bob.clone(), amount: 20 },
			},
		],
	};

	runtime.execute_block(block_1).expect("Invalid Block");

	println!("{:#?}", runtime);
	// println!("{:?}", runtime.balances);
	// println!("{:?}", runtime.system);
}

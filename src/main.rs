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

pub enum RuntimeCall {}

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
		unimplemented!();
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
			self.dispatch(caller, call).map_err(|e| eprintln!("{}", e));
		}

		Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();

	let alice = "Alice".to_string();
	let bob = "Bob".to_string();

	runtime.balances.set_balance(&alice, 100);

	// start emulating a block
	runtime.system.inc_block_number();
	assert!(runtime.system.block_number() == 1);

	// 1st tx
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(&alice, &bob, 50).map_err(|err| eprintln!("{}", err));

	// 2nd tx
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(&alice, &bob, 20).map_err(|err| eprintln!("{}", err));

	// println!("{:?}", runtime);
	println!("{:?}", runtime.balances);
	println!("{:?}", runtime.system);
}

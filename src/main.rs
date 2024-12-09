mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
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

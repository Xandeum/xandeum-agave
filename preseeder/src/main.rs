use bs58;
use solana_ledger::blockstore::Blockstore;
use solana_ledger::blockstore_processor::fill_blockstore_slot_with_ticks;
use solana_sdk::hash::Hash;
use std::path::Path;
use std::str::FromStr; // For parsing integers, etc.

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: preseeder <genesis_hash_in_base58> [ledger-path [ticks-per-slot]]");
        std::process::exit(1);
    }
    let genesis_hash_str = &args[1];
    let ledger_path = args.get(2).cloned().unwrap_or_else(|| "/mnt/ledger".to_string());
    let ticks_per_slot: u64 = args.get(3) .and_then(|s| u64::from_str(s).ok()) .unwrap_or(64);
    let genesis_hash_bytes = bs58::decode(genesis_hash_str) .into_vec() .unwrap_or_else(|_| {
            eprintln!("Invalid genesis hash base58 string");
            std::process::exit(1);
        });
    if genesis_hash_bytes.len() != 32 {
        eprintln!("Genesis hash must decode to 32 bytes");
        std::process::exit(1);
    }
    let genesis_hash = Hash::new(&genesis_hash_bytes);
    let blockstore = Blockstore::open(Path::new(&ledger_path)).expect("Failed to open blockstore at ledger path");
    fill_blockstore_slot_with_ticks(&blockstore, ticks_per_slot, 0, 0, genesis_hash);
    println!("Successfully pre-seeded the ledger at {} with slot 0", ledger_path);
}

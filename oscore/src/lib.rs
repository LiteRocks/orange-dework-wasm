// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Overrides the default panic_fmt
pub fn panic_handler(info: &std::panic::PanicInfo) {
    let panic_msg = format!("{}", info);
    runtime::panic(&panic_msg)
}

pub fn set_panic_handler() {
    std::panic::set_hook(Box::new(panic_handler));
}

///The abi module provides serialization and deserialization methods for different data types in the contract
pub mod abi;

///The runtime module provides an interface to interact with the chain in the contract
pub mod runtime;

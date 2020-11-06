mod env {
    extern "C" {
        pub fn oscore_return(ptr: *const u8, len: u32) -> !;
        pub fn oscore_panic(ptr: *const u8, len: u32) -> !;
        pub fn oscore_input_length() -> u32;
        pub fn oscore_get_input(dst: *mut u8);
        pub fn oscore_sha256(data: *const u8, len: u32, val: *mut u8);
        pub fn oscore_debug(data: *const u8, len: u32);
    }
}

/// Calculate the hash value
/// # Example
///
/// ```no_run
/// # use oscore::runtime;
/// let res = runtime::sha256("test");
/// ```
pub fn sha256(data: impl AsRef<[u8]>) -> [u8; 32] {
    let data = data.as_ref();
    let mut hash = [0; 32];
    unsafe {
        env::oscore_sha256(data.as_ptr(), data.len() as u32, hash.as_mut_ptr());
    }
    hash
}

/// Get input data from transaction or caller contract
/// # Example
///
/// ```no_run
/// # use oscore::runtime;
/// # use oscore::abi::Source;
/// let input = runtime::input();
/// ```
pub fn input() -> Vec<u8> {
    let len = unsafe { env::oscore_input_length() };

    if len == 0 {
        Vec::new()
    } else {
        let mut data = vec![0; len as usize];
        unsafe {
            env::oscore_get_input(data.as_mut_ptr());
        }
        data
    }
}

/// return the result of execution and exit contract execution
/// # Example
///
/// ```no_run
/// # use oscore::runtime;
///   let input = runtime::input();
///   runtime::ret(input.as_slice());
/// ```
pub fn ret(data: &[u8]) -> ! {
    unsafe {
        env::oscore_return(data.as_ptr(), data.len() as u32);
    }
}

/// When the function is executed, all writes to the chain will be cancelled, and the error message will be returned.
///
/// # Example
///
/// ```no_run
/// # use oscore::runtime::panic;
///   panic("panic");
/// ```
pub fn panic(msg: &str) -> ! {
    unsafe {
        env::oscore_panic(msg.as_ptr(), msg.len() as u32);
    }
}

///Used to print the debug information in the contract, which can be seen in the log of the ontology node
/// # Example
/// ```no_run
/// # use oscore::console;
/// console::debug("test");
/// ```
pub fn debug(msg: &str) {
    unsafe {
        env::oscore_debug(msg.as_ptr(), msg.len() as u32);
    }
}

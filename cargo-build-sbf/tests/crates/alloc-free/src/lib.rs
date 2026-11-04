//! Example Rust-based SBF program that allocates without declaring a global
//! allocator, so std's default allocator emits the `sol_alloc_free_` syscall.

#[no_mangle]
/// # SAFETY
/// the runtime passes a valid pointer to the serialized input,
/// which always starts with the 8-byte account count.
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let byte = unsafe { *input };
    // black_box keeps LLVM from eliding the allocation and its syscall.
    let data = std::hint::black_box(Box::new(byte));
    *data as u64
}

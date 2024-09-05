extern "C" {
    fn syscall_keccak_f(w: *mut u64);
}

#[inline]
pub fn keccakf(state: &mut [u64; 25]) {
    unsafe {
        syscall_keccak_f(state.as_mut_ptr());
    }
}

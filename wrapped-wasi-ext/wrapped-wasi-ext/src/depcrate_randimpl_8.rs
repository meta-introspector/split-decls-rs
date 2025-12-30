// Generated macro for impl_8 (impl)
macro_rules! Depcrate_randimpl_8 {
() => {
// Module: crate::rand
// Provides: {"impl_8"}
// Dependencies: {}
impl RngCore for HostInsecureRng { # [inline] fn next_u32 (& mut self) -> u32 { wasi :: random :: insecure :: get_insecure_random_u64 () as _ } # [inline] fn next_u64 (& mut self) -> u64 { wasi :: random :: insecure :: get_insecure_random_u64 () } fn fill_bytes (& mut self , dest : & mut [u8]) { let n = dest . len () ; if usize :: BITS <= u64 :: BITS || n <= u64 :: MAX as _ { dest . copy_from_slice (& wasi :: random :: insecure :: get_insecure_random_bytes (n as _)) ; } else { let (head , tail) = dest . split_at_mut (u64 :: MAX as _) ; head . copy_from_slice (& wasi :: random :: insecure :: get_insecure_random_bytes (u64 :: MAX)) ; self . fill_bytes (tail) ; } } # [inline] fn try_fill_bytes (& mut self , dest : & mut [u8]) -> Result < () , rand :: Error > { self . fill_bytes (dest) ; Ok (()) } }
};
}

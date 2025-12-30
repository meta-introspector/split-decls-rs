// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl XxHash64 { # [inline] pub fn oneshot (seed : u64 , data : & [u8]) -> u64 { unsafe { XXH64 (data . as_ptr () . cast () , data . len () , seed) } } # [inline] pub fn with_seed (seed : u64) -> Self { let state = unsafe { let state = XXH64_createState () ; XXH64_reset (state , seed) ; state } ; Self (state) } # [inline] pub fn write (& mut self , data : & [u8]) { let retval = unsafe { XXH64_update (self . 0 , data . as_ptr () . cast () , data . len ()) } ; assert_eq ! (retval , XXH_OK) ; } # [inline] pub fn finish (& mut self) -> u64 { unsafe { XXH64_digest (self . 0) } } }
};
}

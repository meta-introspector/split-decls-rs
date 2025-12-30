// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl XxHash32 { # [inline] pub fn oneshot (seed : u32 , data : & [u8]) -> u32 { unsafe { XXH32 (data . as_ptr () . cast () , data . len () , seed) } } # [inline] pub fn with_seed (seed : u32) -> Self { let state = unsafe { let state = XXH32_createState () ; XXH32_reset (state , seed) ; state } ; Self (state) } # [inline] pub fn write (& mut self , data : & [u8]) { let retval = unsafe { XXH32_update (self . 0 , data . as_ptr () . cast () , data . len ()) } ; assert_eq ! (retval , XXH_OK) ; } # [inline] pub fn finish (& mut self) -> u32 { unsafe { XXH32_digest (self . 0) } } }
};
}

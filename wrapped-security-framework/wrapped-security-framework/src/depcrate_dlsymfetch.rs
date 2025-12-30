// Generated macro for fetch (function)
macro_rules! Depcrate_dlsymfetch {
() => {
// Module: crate::dlsym
// Provides: {"fetch"}
// Dependencies: {}
unsafe fn fetch (name : & str) -> usize { assert_eq ! (name . as_bytes () [name . len () - 1] , 0) ; match libc :: dlsym (libc :: RTLD_DEFAULT , name . as_ptr () as * const _) as usize { 0 => 1 , n => n , } }
};
}

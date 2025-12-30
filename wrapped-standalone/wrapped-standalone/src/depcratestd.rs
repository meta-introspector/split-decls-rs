// Generated macro for std (function)
macro_rules! Depcratestd {
() => {
// Module: crate
// Provides: {"std"}
// Dependencies: {}
# [test] fn std () { unsafe { assert_eq ! (b_std :: CloseHandle (std :: ptr :: null_mut ()) , 0) ; let mut buffer = [0u8 ; 8] ; assert ! (b_std :: RtlGenRandom (buffer . as_mut_ptr () as _ , buffer . len () as _)) ; assert_ne ! (& buffer , & [0u8 ; 8]) ; } }
};
}

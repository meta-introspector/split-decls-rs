// Generated macro for as_block (function)
macro_rules! Depcrateas_block {
() => {
// Module: crate
// Provides: {"as_block"}
// Dependencies: {}
# [inline (always)] fn as_block (input : & [u8]) -> & [u8 ; 64] { unsafe { assert ! (input . len () == 64) ; let arr : & [u8 ; 64] = & * (input . as_ptr () as * const [u8 ; 64]) ; arr } }
};
}

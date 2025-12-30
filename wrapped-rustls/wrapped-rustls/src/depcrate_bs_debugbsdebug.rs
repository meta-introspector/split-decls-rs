// Generated macro for BsDebug (struct)
macro_rules! Depcrate_bs_debugBsDebug {
() => {
// Module: crate::bs_debug
// Provides: {"BsDebug"}
// Dependencies: {}
# [doc = " Alternative implementation of `fmt::Debug` for byte slice."] # [doc = ""] # [doc = " Standard `Debug` implementation for `[u8]` is comma separated"] # [doc = " list of numbers. Since large amount of byte strings are in fact"] # [doc = " ASCII strings or contain a lot of ASCII strings (e. g. HTTP),"] # [doc = " it is convenient to print strings as ASCII when possible."] # [doc = ""] # [doc = " This struct wraps `&[u8]` just to override `fmt::Debug`."] # [doc = ""] # [doc = " `BsDebug` is not a part of public API of bytes crate."] pub (crate) struct BsDebug < 'a > (pub (crate) & 'a [u8]) ;
};
}

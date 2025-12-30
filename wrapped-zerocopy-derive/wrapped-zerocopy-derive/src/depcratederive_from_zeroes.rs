// Generated macro for derive_from_zeroes (function)
macro_rules! Depcratederive_from_zeroes {
() => {
// Module: crate
// Provides: {"derive_from_zeroes"}
// Dependencies: {}
# [doc = " Deprecated: prefer [`FromZeros`] instead."] # [deprecated (since = "0.8.0" , note = "`FromZeroes` was renamed to `FromZeros`")] # [doc (hidden)] # [proc_macro_derive (FromZeroes)] pub fn derive_from_zeroes (ts : proc_macro :: TokenStream) -> proc_macro :: TokenStream { derive_from_zeros (ts) }
};
}

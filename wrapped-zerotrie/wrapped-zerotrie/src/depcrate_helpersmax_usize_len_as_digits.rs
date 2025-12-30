// Generated macro for MAX_USIZE_LEN_AS_DIGITS (const)
macro_rules! Depcrate_helpersMAX_USIZE_LEN_AS_DIGITS {
() => {
// Module: crate::helpers
// Provides: {"MAX_USIZE_LEN_AS_DIGITS"}
// Dependencies: {}
# [doc = " The maximum number of base-10 digits required for rendering a usize."] # [doc = " Note: 24/10 is an approximation of 8*log10(2)"] pub (crate) const MAX_USIZE_LEN_AS_DIGITS : usize = core :: mem :: size_of :: < usize > () * 24 / 10 + 1 ;
};
}

// Generated macro for sval2 (function)
macro_rules! Depcrate_internal_serde_v1sval2 {
() => {
// Module: crate::internal::serde::v1
// Provides: {"sval2"}
// Dependencies: {}
# [cfg (feature = "sval2")] pub (in crate :: internal) fn sval2 < 'sval , S : value_bag_sval2 :: lib :: Stream < 'sval > + ? Sized > (s : & mut S , v : & dyn Serialize ,) -> Result < () , Error > { value_bag_sval2 :: serde1 :: stream (s , v) . map_err (Error :: from_sval2) }
};
}

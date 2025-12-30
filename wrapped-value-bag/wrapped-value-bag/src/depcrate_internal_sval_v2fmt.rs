// Generated macro for fmt (function)
macro_rules! Depcrate_internal_sval_v2fmt {
() => {
// Module: crate::internal::sval::v2
// Provides: {"fmt"}
// Dependencies: {}
pub (in crate :: internal) fn fmt (f : & mut fmt :: Formatter , v : & dyn Value) -> Result < () , Error > { value_bag_sval2 :: fmt :: stream_to_fmt (f , v) ? ; Ok (()) }
};
}

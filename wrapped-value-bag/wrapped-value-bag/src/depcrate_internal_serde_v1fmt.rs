// Generated macro for fmt (function)
macro_rules! Depcrate_internal_serde_v1fmt {
() => {
// Module: crate::internal::serde::v1
// Provides: {"fmt"}
// Dependencies: {}
pub (in crate :: internal) fn fmt (f : & mut fmt :: Formatter , v : & dyn Serialize) -> Result < () , Error > { fmt :: Debug :: fmt (& value_bag_serde1 :: fmt :: to_debug (v) , f) ? ; Ok (()) }
};
}

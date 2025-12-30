// Generated macro for serde1 (function)
macro_rules! Depcrate_internal_sval_v2serde1 {
() => {
// Module: crate::internal::sval::v2
// Provides: {"serde1"}
// Dependencies: {}
# [cfg (feature = "serde1")] pub (in crate :: internal) fn serde1 < S > (s : S , v : & dyn Value) -> Result < S :: Ok , S :: Error > where S : value_bag_serde1 :: lib :: Serializer , { value_bag_sval2 :: serde1 :: serialize (s , v) }
};
}

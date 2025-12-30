// Generated macro for impl_50 (impl)
macro_rules! Depcrate_unvalidatedimpl_50 {
() => {
// Module: crate::unvalidated
// Provides: {"impl_50"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < const N : usize > serde_core :: Serialize for UnvalidatedTinyAsciiStr < N > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: Serializer , { use serde_core :: ser :: Error ; self . try_into_tinystr () . map_err (| _ | S :: Error :: custom ("invalid ascii in UnvalidatedTinyAsciiStr")) ? . serialize (serializer) } }
};
}

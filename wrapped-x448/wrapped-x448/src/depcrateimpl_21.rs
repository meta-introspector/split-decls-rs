// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (all (feature = "static_secrets" , feature = "serde"))] impl serdect :: serde :: Serialize for StaticSecret { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : serdect :: serde :: Serializer , { serdect :: array :: serialize_hex_lower_or_bin (& self . 0 , s) } }
};
}

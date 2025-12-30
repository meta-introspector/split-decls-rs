// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg (all (feature = "static_secrets" , feature = "serde"))] impl < 'de > serdect :: serde :: Deserialize < 'de > for StaticSecret { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : serdect :: serde :: Deserializer < 'de > , { let mut bytes = Array :: default () ; serdect :: array :: deserialize_hex_or_bin (& mut bytes , d) ? ; Ok (Self (bytes)) } }
};
}

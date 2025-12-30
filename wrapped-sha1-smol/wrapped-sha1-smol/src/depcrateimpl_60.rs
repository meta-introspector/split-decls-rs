// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: de :: Deserialize < 'de > for Digest { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct V ; impl < 'de > serde :: de :: Visitor < 'de > for V { type Value = Digest ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("SHA-1 hash") } fn visit_str < E > (self , value : & str) -> Result < Digest , E > where E : serde :: de :: Error , { value . parse () . map_err (| _ | { serde :: de :: Error :: invalid_value (serde :: de :: Unexpected :: Str (value) , & self) }) } } deserializer . deserialize_str (V) } }
};
}

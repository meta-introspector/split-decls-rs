// Generated macro for impl_494 (impl)
macro_rules! Depcrate_zerovec_serdeimpl_494 {
() => {
// Module: crate::zerovec::serde
// Provides: {"impl_494"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] impl < 'de , 'a , T > Deserialize < 'de > for ZeroVec < 'a , T > where T : 'de + Deserialize < 'de > + AsULE , 'de : 'a , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let visitor = ZeroVecVisitor :: default () ; if deserializer . is_human_readable () { deserializer . deserialize_seq (visitor) } else { deserializer . deserialize_bytes (visitor) } } }
};
}

// Generated macro for impl_461 (impl)
macro_rules! Depcrate_varzerovec_serdeimpl_461 {
() => {
// Module: crate::varzerovec::serde
// Provides: {"impl_461"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] # [cfg (feature = "alloc")] impl < 'de , 'a , T , F > Deserialize < 'de > for VarZeroVec < 'a , T , F > where T : VarULE + ? Sized , Box < T > : Deserialize < 'de > , F : VarZeroVecFormat , 'de : 'a , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { deserializer . deserialize_seq (VarZeroVecHumanVisitor :: < T , F > :: default ()) } else { deserializer . deserialize_bytes (VarZeroVecVisitor :: < T , F > :: default ()) } } }
};
}

// Generated macro for impl_462 (impl)
macro_rules! Depcrate_varzerovec_serdeimpl_462 {
() => {
// Module: crate::varzerovec::serde
// Provides: {"impl_462"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] # [cfg (not (feature = "alloc"))] impl < 'de , 'a , T , F > Deserialize < 'de > for VarZeroVec < 'a , T , F > where T : VarULE + ? Sized , F : VarZeroVecFormat , 'de : 'a , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_bytes (VarZeroVecVisitor :: < T , F > :: default ()) } }
};
}

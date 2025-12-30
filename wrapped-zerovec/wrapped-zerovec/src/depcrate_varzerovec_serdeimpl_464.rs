// Generated macro for impl_464 (impl)
macro_rules! Depcrate_varzerovec_serdeimpl_464 {
() => {
// Module: crate::varzerovec::serde
// Provides: {"impl_464"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] # [cfg (feature = "alloc")] impl < 'de , T , F > Deserialize < 'de > for Box < VarZeroSlice < T , F > > where T : VarULE + ? Sized , Box < T > : Deserialize < 'de > , F : VarZeroVecFormat , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let deserialized = VarZeroVec :: < T , F > :: deserialize (deserializer) ? ; Ok (deserialized . to_boxed ()) } }
};
}

// Generated macro for impl_463 (impl)
macro_rules! Depcrate_varzerovec_serdeimpl_463 {
() => {
// Module: crate::varzerovec::serde
// Provides: {"impl_463"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] impl < 'de , 'a , T , F > Deserialize < 'de > for & 'a VarZeroSlice < T , F > where T : VarULE + ? Sized , F : VarZeroVecFormat , 'de : 'a , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { Err (de :: Error :: custom ("&VarZeroSlice cannot be deserialized from human-readable formats" ,)) } else { let bytes = < & [u8] > :: deserialize (deserializer) ? ; VarZeroSlice :: < T , F > :: parse_bytes (bytes) . map_err (de :: Error :: custom) } } }
};
}

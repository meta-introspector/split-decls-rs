// Generated macro for impl_497 (impl)
macro_rules! Depcrate_zerovec_serdeimpl_497 {
() => {
// Module: crate::zerovec::serde
// Provides: {"impl_497"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] impl < 'de , 'a , T > Deserialize < 'de > for & 'a ZeroSlice < T > where T : Deserialize < 'de > + AsULE + 'static , 'de : 'a , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { Err (de :: Error :: custom ("&ZeroSlice cannot be deserialized from human-readable formats" ,)) } else { let deserialized : ZeroVec < 'a , T > = ZeroVec :: deserialize (deserializer) ? ; let borrowed = if let Some (b) = deserialized . as_maybe_borrowed () { b } else { return Err (de :: Error :: custom ("&ZeroSlice can only deserialize in zero-copy ways" ,)) ; } ; Ok (borrowed) } } }
};
}

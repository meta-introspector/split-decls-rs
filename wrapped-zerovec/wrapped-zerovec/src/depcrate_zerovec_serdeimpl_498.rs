// Generated macro for impl_498 (impl)
macro_rules! Depcrate_zerovec_serdeimpl_498 {
() => {
// Module: crate::zerovec::serde
// Provides: {"impl_498"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] impl < T > Serialize for ZeroSlice < T > where T : Serialize + AsULE , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . as_zerovec () . serialize (serializer) } }
};
}

// Generated macro for impl_466 (impl)
macro_rules! Depcrate_varzerovec_serdeimpl_466 {
() => {
// Module: crate::varzerovec::serde
// Provides: {"impl_466"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] # [cfg (feature = "serde")] impl < T , F > Serialize for VarZeroSlice < T , F > where T : Serialize + VarULE + ? Sized , F : VarZeroVecFormat , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . as_varzerovec () . serialize (serializer) } }
};
}

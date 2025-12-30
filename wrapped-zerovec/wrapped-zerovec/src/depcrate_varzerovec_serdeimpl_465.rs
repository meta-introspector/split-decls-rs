// Generated macro for impl_465 (impl)
macro_rules! Depcrate_varzerovec_serdeimpl_465 {
() => {
// Module: crate::varzerovec::serde
// Provides: {"impl_465"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] # [cfg (feature = "serde")] impl < T , F > Serialize for VarZeroVec < '_ , T , F > where T : Serialize + VarULE + ? Sized , F : VarZeroVecFormat , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for value in self . iter () { seq . serialize_element (value) ? ; } seq . end () } else { serializer . serialize_bytes (self . as_bytes ()) } } }
};
}

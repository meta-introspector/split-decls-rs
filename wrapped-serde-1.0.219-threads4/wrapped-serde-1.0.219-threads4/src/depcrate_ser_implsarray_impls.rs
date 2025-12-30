// Generated macro for array_impls (macro)
macro_rules! Depcrate_ser_implsarray_impls {
() => {
// Module: crate::ser::impls
// Provides: {"array_impls"}
// Dependencies: {}
macro_rules ! array_impls { ($ ($ len : tt) +) => { $ (impl < T > Serialize for [T ; $ len] where T : Serialize , { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut seq = tri ! (serializer . serialize_tuple ($ len)) ; for e in self { tri ! (seq . serialize_element (e)) ; } seq . end () } }) + } }
};
}

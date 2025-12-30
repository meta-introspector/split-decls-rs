// Generated macro for primitive_impl (macro)
macro_rules! Depcrate_ser_implsprimitive_impl {
() => {
// Module: crate::ser::impls
// Provides: {"primitive_impl"}
// Dependencies: {}
macro_rules ! primitive_impl { ($ ty : ident , $ method : ident $ ($ cast : tt) *) => { impl Serialize for $ ty { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer .$ method (* self $ ($ cast) *) } } } }
};
}

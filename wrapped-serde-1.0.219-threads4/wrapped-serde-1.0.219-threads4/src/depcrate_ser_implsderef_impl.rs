// Generated macro for deref_impl (macro)
macro_rules! Depcrate_ser_implsderef_impl {
() => {
// Module: crate::ser::impls
// Provides: {"deref_impl"}
// Dependencies: {}
macro_rules ! deref_impl { ($ (# [$ attr : meta]) * <$ ($ desc : tt) +) => { $ (# [$ attr]) * impl <$ ($ desc) + { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { (** self) . serialize (serializer) } } } ; }
};
}

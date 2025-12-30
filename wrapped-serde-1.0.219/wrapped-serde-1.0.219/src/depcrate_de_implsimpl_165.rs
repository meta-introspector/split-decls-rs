// Generated macro for impl_165 (impl)
macro_rules! Depcrate_de_implsimpl_165 {
() => {
// Module: crate::de::impls
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for BoolVisitor { type Value = bool ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a boolean") } fn visit_bool < E > (self , v : bool) -> Result < Self :: Value , E > where E : Error , { Ok (v) } }
};
}

// Generated macro for impl_161 (impl)
macro_rules! Depcrate_de_implsimpl_161 {
() => {
// Module: crate::de::impls
// Provides: {"impl_161"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for UnitVisitor { type Value = () ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("unit") } fn visit_unit < E > (self) -> Result < Self :: Value , E > where E : Error , { Ok (()) } }
};
}

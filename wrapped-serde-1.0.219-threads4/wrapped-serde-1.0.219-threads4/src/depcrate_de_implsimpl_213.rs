// Generated macro for impl_213 (impl)
macro_rules! Depcrate_de_implsimpl_213 {
() => {
// Module: crate::de::impls
// Provides: {"impl_213"}
// Dependencies: {}
impl < 'de , T > Visitor < 'de > for PhantomDataVisitor < T > where T : ? Sized , { type Value = PhantomData < T > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("unit") } # [inline] fn visit_unit < E > (self) -> Result < Self :: Value , E > where E : Error , { Ok (PhantomData) } }
};
}

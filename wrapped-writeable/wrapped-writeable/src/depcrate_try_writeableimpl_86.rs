// Generated macro for impl_86 (impl)
macro_rules! Depcrate_try_writeableimpl_86 {
() => {
// Module: crate::try_writeable
// Provides: {"impl_86"}
// Dependencies: {}
impl < T > fmt :: Display for TryWriteableInfallibleAsWriteable < T > where T : TryWriteable < Error = Infallible > , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . write_to (f) } }
};
}

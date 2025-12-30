// Generated macro for impl_237 (impl)
macro_rules! Depcrate_reactor_registrarimpl_237 {
() => {
// Module: crate::reactor::registrar
// Provides: {"impl_237"}
// Dependencies: {}
impl < R , CODEC > fmt :: Debug for ReactorRegistrar < R , CODEC > where R : Reactor + 'static , CODEC : Codec + 'static , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ReactorRegistrar<_>") . finish () } }
};
}

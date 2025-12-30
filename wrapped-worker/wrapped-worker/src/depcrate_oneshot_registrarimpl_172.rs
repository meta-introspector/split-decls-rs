// Generated macro for impl_172 (impl)
macro_rules! Depcrate_oneshot_registrarimpl_172 {
() => {
// Module: crate::oneshot::registrar
// Provides: {"impl_172"}
// Dependencies: {}
impl < T , CODEC > fmt :: Debug for OneshotRegistrar < T , CODEC > where T : Oneshot + 'static , CODEC : Codec + 'static , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OneshotRegistrar<_>") . finish () } }
};
}

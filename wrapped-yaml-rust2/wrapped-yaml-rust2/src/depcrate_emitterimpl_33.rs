// Generated macro for impl_33 (impl)
macro_rules! Depcrate_emitterimpl_33 {
() => {
// Module: crate::emitter
// Provides: {"impl_33"}
// Dependencies: {}
impl Display for EmitError { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match * self { EmitError :: FmtError (ref err) => Display :: fmt (err , formatter) , } } }
};
}

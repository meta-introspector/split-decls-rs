// Generated macro for impl_27 (impl)
macro_rules! Depcrate_errorimpl_27 {
() => {
// Module: crate::error
// Provides: {"impl_27"}
// Dependencies: {}
impl std :: error :: Error for Error { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match & self . 0 { Kind :: CreateFile { ref source , .. } => Some (source) , Kind :: FlushFile (ref source) => Some (source) , } } }
};
}

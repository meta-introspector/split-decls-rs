// Generated macro for impl_358 (impl)
macro_rules! Depcrate_compiler_argsimpl_358 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_358"}
// Dependencies: {}
impl Display for ArgToStringError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s = match self { ArgToStringError :: FailedPathTransform (p) => { format ! ("Path {:?} could not be transformed" , p) } ArgToStringError :: InvalidUnicode (s) => { format ! ("String {:?} contained invalid unicode" , s) } } ; write ! (f , "{}" , s) } }
};
}

// Generated macro for impl_355 (impl)
macro_rules! Depcrate_compiler_argsimpl_355 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_355"}
// Dependencies: {}
impl Display for ArgParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s = match self { ArgParseError :: UnexpectedEndOfArgs => "Unexpected end of args" . into () , ArgParseError :: InvalidUnicode (s) => format ! ("String {:?} contained invalid unicode" , s) , ArgParseError :: Other (s) => format ! ("Arg-specific parsing failed: {}" , s) , } ; write ! (f , "{}" , s) } }
};
}

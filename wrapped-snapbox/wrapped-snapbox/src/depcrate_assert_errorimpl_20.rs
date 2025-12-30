// Generated macro for impl_20 (impl)
macro_rules! Depcrate_assert_errorimpl_20 {
() => {
// Module: crate::assert::error
// Provides: {"impl_20"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { writeln ! (f , "{}" , self . inner) ? ; if let Some (backtrace) = self . backtrace . as_ref () { writeln ! (f) ? ; writeln ! (f , "Backtrace:") ? ; writeln ! (f , "{backtrace}") ? ; } Ok (()) } }
};
}

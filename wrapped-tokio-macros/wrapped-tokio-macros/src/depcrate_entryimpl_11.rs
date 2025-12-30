// Generated macro for impl_11 (impl)
macro_rules! Depcrate_entryimpl_11 {
() => {
// Module: crate::entry
// Provides: {"impl_11"}
// Dependencies: {}
impl UnhandledPanic { fn from_str (s : & str) -> Result < UnhandledPanic , String > { match s { "ignore" => Ok (UnhandledPanic :: Ignore) , "shutdown_runtime" => Ok (UnhandledPanic :: ShutdownRuntime) , _ => Err (format ! ("No such unhandled panic behavior `{s}`. The unhandled panic behaviors are `ignore` and `shutdown_runtime`.")) , } } fn into_tokens (self , crate_path : & TokenStream) -> TokenStream { match self { UnhandledPanic :: Ignore => quote ! { # crate_path :: runtime :: UnhandledPanic :: Ignore } , UnhandledPanic :: ShutdownRuntime => { quote ! { # crate_path :: runtime :: UnhandledPanic :: ShutdownRuntime } } } } }
};
}

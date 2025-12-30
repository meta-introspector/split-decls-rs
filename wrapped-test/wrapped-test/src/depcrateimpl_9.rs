// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a > TokenBuf < 'a > { # [doc = "\n    Create a new, empty token buffer.\n    "] pub fn new () -> Self { TokenBuf { tokens : Vec :: new () } } # [doc = "\n    Get the underlying tokens in this buffer.\n    "] pub fn as_tokens (& self) -> & [Token < 'a >] { & self . tokens } fn push (& mut self , token : Token < 'a >) { self . tokens . push (token) ; } # [track_caller] fn fail < T : ? Sized > (& self) { panic ! ("the `impl sval::Value for {}` is invalid\nstreamed to:\n  `{}`\nraw:\n  `{:?}`" , type_name ::< T > () , sval_fmt :: stream_to_string (AsValue (& self . tokens)) , self . tokens) ; } }
};
}

// Generated macro for impl_87 (impl)
macro_rules! Depcrate_serimpl_87 {
() => {
// Module: crate::ser
// Provides: {"impl_87"}
// Dependencies: {}
impl error :: Error for Error { fn description (& self) -> & str { match * self { Error :: Custom (ref msg) => msg , Error :: Utf8 (ref err) => error :: Error :: description (err) , } } # [doc = " The lower-level cause of this error, in the case of a `Utf8` error."] fn cause (& self) -> Option < & dyn error :: Error > { match * self { Error :: Custom (_) => None , Error :: Utf8 (ref err) => Some (err) , } } # [doc = " The lower-level source of this error, in the case of a `Utf8` error."] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { Error :: Custom (_) => None , Error :: Utf8 (ref err) => Some (err) , } } }
};
}

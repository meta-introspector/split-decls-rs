// Generated macro for impl_807 (impl)
macro_rules! Depcrate_resultimpl_807 {
() => {
// Module: crate::result
// Provides: {"impl_807"}
// Dependencies: {}
impl < T , E > QuicResultExt < T , E > for Result < T , E > { # [inline] fn into_io (self) -> io :: Result < T > where E : Into < BoxError > , { self . map_err (io :: Error :: other) } }
};
}

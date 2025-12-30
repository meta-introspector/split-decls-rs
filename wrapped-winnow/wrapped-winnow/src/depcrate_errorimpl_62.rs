// Generated macro for impl_62 (impl)
macro_rules! Depcrate_errorimpl_62 {
() => {
// Module: crate::error
// Provides: {"impl_62"}
// Dependencies: {}
impl < I , EXT , E > FromExternalError < I , EXT > for ErrMode < E > where E : FromExternalError < I , EXT > , { # [inline (always)] fn from_external_error (input : & I , e : EXT) -> Self { ErrMode :: Backtrack (E :: from_external_error (input , e)) } }
};
}

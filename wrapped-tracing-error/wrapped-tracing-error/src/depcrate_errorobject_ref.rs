// Generated macro for object_ref (function)
macro_rules! Depcrate_errorobject_ref {
() => {
// Module: crate::error
// Provides: {"object_ref"}
// Dependencies: {}
unsafe fn object_ref < E > (e : & ErrorImpl < Erased >) -> & (dyn Error + Send + Sync + 'static) where E : Error + Send + Sync + 'static , { & (* (e as * const ErrorImpl < Erased > as * const ErrorImpl < E >)) . error }
};
}

// Generated macro for impl_88 (impl)
macro_rules! Depcrate_serimpl_88 {
() => {
// Module: crate::ser
// Provides: {"impl_88"}
// Dependencies: {}
impl ser :: Error for Error { fn custom < T : fmt :: Display > (msg : T) -> Self { Error :: Custom (format ! ("{}" , msg) . into ()) } }
};
}

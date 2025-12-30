// Generated macro for macro_159 (macro)
macro_rules! Depcrate_filter_futuremacro_159 {
() => {
// Module: crate::filter::future
// Provides: {"macro_159"}
// Dependencies: {}
opaque_future ! { # [doc = " Filtered response future from [`Filter`] services."] # [doc = ""] # [doc = " [`Filter`]: crate::filter::Filter"] pub type ResponseFuture < R , F > = futures_util :: future :: Either < std :: future :: Ready < Result < R , crate :: BoxError >>, futures_util :: future :: ErrInto < F , crate :: BoxError > >; }
};
}

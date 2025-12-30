// Generated macro for impl_396 (impl)
macro_rules! Depcrate_report_colorimpl_396 {
() => {
// Module: crate::report::color
// Provides: {"impl_396"}
// Dependencies: {}
impl < D : std :: fmt :: Display > std :: fmt :: Display for Styled < D > { # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . style . render ()) ? ; self . display . fmt (f) ? ; write ! (f , "{}" , self . style . render_reset ()) ? ; Ok (()) } }
};
}

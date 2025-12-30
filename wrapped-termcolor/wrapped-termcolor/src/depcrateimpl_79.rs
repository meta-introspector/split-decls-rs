// Generated macro for impl_79 (impl)
macro_rules! Depcrateimpl_79 {
() => {
// Module: crate
// Provides: {"impl_79"}
// Dependencies: {}
impl < W : io :: Write > LossyStandardStream < W > { # [cfg (not (windows))] fn new (wtr : W) -> LossyStandardStream < W > { LossyStandardStream { wtr } } # [cfg (windows)] fn new (wtr : W) -> LossyStandardStream < W > { let is_console = wincon :: Console :: stdout () . is_ok () || wincon :: Console :: stderr () . is_ok () ; LossyStandardStream { wtr , is_console } } # [cfg (not (windows))] fn wrap < Q : io :: Write > (& self , wtr : Q) -> LossyStandardStream < Q > { LossyStandardStream :: new (wtr) } # [cfg (windows)] fn wrap < Q : io :: Write > (& self , wtr : Q) -> LossyStandardStream < Q > { LossyStandardStream { wtr , is_console : self . is_console } } fn get_ref (& self) -> & W { & self . wtr } }
};
}

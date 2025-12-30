// Generated macro for impl_35 (impl)
macro_rules! Depcrate_pass_managerimpl_35 {
() => {
// Module: crate::pass_manager
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'tcx , T > MirPass < 'tcx > for Lint < T > where T : MirLint < 'tcx > , { fn name (& self) -> & 'static str { self . 0 . name () } fn is_enabled (& self , sess : & Session) -> bool { self . 0 . is_enabled (sess) } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { self . 0 . run_lint (tcx , body) } fn is_mir_dump_enabled (& self) -> bool { false } fn is_required (& self) -> bool { true } }
};
}

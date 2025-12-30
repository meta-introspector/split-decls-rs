// Generated macro for declare_late_lint_pass (macro)
macro_rules! Depcrate_passesdeclare_late_lint_pass {
() => {
// Module: crate::passes
// Provides: {"declare_late_lint_pass"}
// Dependencies: {}
# [doc = " Trait for types providing lint checks."] # [doc = ""] # [doc = " Each `check` method checks a single syntax node, and should not"] # [doc = " invoke methods recursively (unlike `Visitor`). By default they"] # [doc = " do nothing."] macro_rules ! declare_late_lint_pass { ([] , [$ ($ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : ident : $ arg : ty) ,*) ;) *]) => (pub trait LateLintPass <'tcx >: LintPass { $ (# [inline (always)] fn $ name (& mut self , _ : & LateContext <'tcx >, $ (_ : $ arg) ,*) { }) * }) }
};
}

// Generated macro for impl_157 (impl)
macro_rules! Depcrateimpl_157 {
() => {
// Module: crate
// Provides: {"impl_157"}
// Dependencies: {}
impl LintExpectationId { pub fn is_stable (& self) -> bool { match self { LintExpectationId :: Unstable { .. } => false , LintExpectationId :: Stable { .. } => true , } } pub fn get_lint_index (& self) -> Option < u16 > { let (LintExpectationId :: Unstable { lint_index , .. } | LintExpectationId :: Stable { lint_index , .. }) = self ; * lint_index } pub fn set_lint_index (& mut self , new_lint_index : Option < u16 >) { let (LintExpectationId :: Unstable { lint_index , .. } | LintExpectationId :: Stable { lint_index , .. }) = self ; * lint_index = new_lint_index } }
};
}

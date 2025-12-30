// Generated macro for impl_17 (impl)
macro_rules! Depcrate_cnfimpl_17 {
() => {
// Module: crate::cnf
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a , F , V > Iterator for NewVarIter < 'a , F , V > where F : ExtendFormula , V : From < Var > , { type Item = V ; fn next (& mut self) -> Option < V > { if self . vars_left == 0 { None } else { let var = self . formula . new_var () ; self . vars_left -= 1 ; Some (V :: from (var)) } } }
};
}

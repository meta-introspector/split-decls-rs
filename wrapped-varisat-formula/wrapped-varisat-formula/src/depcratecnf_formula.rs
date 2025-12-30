// Generated macro for cnf_formula (macro)
macro_rules! Depcratecnf_formula {
() => {
// Module: crate
// Provides: {"cnf_formula"}
// Dependencies: {}
# [doc = " Shortcut for tests"] # [cfg (any (test , feature = "internal-testing"))] # [doc (hidden)] # [macro_export] macro_rules ! cnf_formula { ($ ($ t : tt) *) => { $ crate :: cnf :: CnfFormula :: from ($ crate :: cnf ! [$ ($ t) *] . iter () . cloned ()) } ; }
};
}

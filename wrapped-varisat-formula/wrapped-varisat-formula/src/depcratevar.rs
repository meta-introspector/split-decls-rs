// Generated macro for var (macro)
macro_rules! Depcratevar {
() => {
// Module: crate
// Provides: {"var"}
// Dependencies: {}
# [doc = " Shortcut for tests"] # [cfg (any (test , feature = "internal-testing"))] # [doc (hidden)] # [macro_export] macro_rules ! var { ($ x : expr) => { $ crate :: lit :: Var :: from_dimacs ($ x) } ; }
};
}

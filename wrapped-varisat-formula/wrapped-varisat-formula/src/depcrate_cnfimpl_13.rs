// Generated macro for impl_13 (impl)
macro_rules! Depcrate_cnfimpl_13 {
() => {
// Module: crate::cnf
// Provides: {"impl_13"}
// Dependencies: {}
impl fmt :: Debug for CnfFormula { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& self . var_count () , f) ? ; f . debug_list () . entries (self . iter ()) . finish () } }
};
}

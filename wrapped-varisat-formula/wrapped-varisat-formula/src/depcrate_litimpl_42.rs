// Generated macro for impl_42 (impl)
macro_rules! Depcrate_litimpl_42 {
() => {
// Module: crate::lit
// Provides: {"impl_42"}
// Dependencies: {}
# [doc = " Uses the 1-based DIMACS CNF encoding."] impl fmt :: Debug for Lit { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , self . to_dimacs ()) } }
};
}

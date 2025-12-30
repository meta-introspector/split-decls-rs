// Generated macro for impl_74 (impl)
macro_rules! Depcrate_hygieneimpl_74 {
() => {
// Module: crate::hygiene
// Provides: {"impl_74"}
// Dependencies: {}
impl std :: fmt :: Debug for SyntaxContext { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if f . alternate () { fmt :: Display :: fmt (self , f) } else { f . debug_tuple ("SyntaxContext") . field (& self . 0) . finish () } } }
};
}

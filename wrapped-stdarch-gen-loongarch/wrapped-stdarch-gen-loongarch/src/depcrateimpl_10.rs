// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl std :: fmt :: Display for Lines { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> fmt :: Result { for line in self . lines . iter () { write ! (f , "\n{:width$}{line}" , "" , width = self . indent) ? ; } Ok (()) } }
};
}

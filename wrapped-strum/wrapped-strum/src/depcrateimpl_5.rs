// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: fmt :: Display for ParseError { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> Result < () , std :: fmt :: Error > { match self { ParseError :: VariantNotFound => write ! (f , "Matching variant not found") , } } }
};
}

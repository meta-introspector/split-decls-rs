// Generated macro for impl_295 (impl)
macro_rules! Depcrate_reprimpl_295 {
() => {
// Module: crate::repr
// Provides: {"impl_295"}
// Dependencies: {}
# [cfg (feature = "display")] impl < T > std :: fmt :: Display for Formatted < T > where T : ValueRepr , { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { crate :: encode :: encode_formatted (self , f , None , ("" , "")) } }
};
}

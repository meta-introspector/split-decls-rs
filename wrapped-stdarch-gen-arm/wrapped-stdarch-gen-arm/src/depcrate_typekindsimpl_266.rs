// Generated macro for impl_266 (impl)
macro_rules! Depcrate_typekindsimpl_266 {
() => {
// Module: crate::typekinds
// Provides: {"impl_266"}
// Dependencies: {}
impl fmt :: Display for TypeKindOptions { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . f . then (| | write ! (f , "f")) . transpose () ? ; self . s . then (| | write ! (f , "s")) . transpose () ? ; self . u . then (| | write ! (f , "u")) . transpose () . map (| _ | ()) } }
};
}

// Generated macro for impl_54 (impl)
macro_rules! Depcrate_metadataimpl_54 {
() => {
// Module: crate::metadata
// Provides: {"impl_54"}
// Dependencies: {}
impl fmt :: Display for ExpectedMetadata { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref name) = self . name { write ! (f , " named `{}`" , name) ? ; } if let Some (ref level) = self . level { write ! (f , " at the `{:?}` level" , level) ? ; } if let Some (ref target) = self . target { write ! (f , " with target `{}`" , target) ? ; } Ok (()) } }
};
}

// Generated macro for impl_144 (impl)
macro_rules! Depcrate_udiffimpl_144 {
() => {
// Module: crate::udiff
// Provides: {"impl_144"}
// Dependencies: {}
impl fmt :: Display for UnifiedHunkHeader { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "@@ -{} +{} @@" , & self . old_range , & self . new_range) } }
};
}

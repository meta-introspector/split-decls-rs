// Generated macro for impl_749 (impl)
macro_rules! Depcrate_fsimpl_749 {
() => {
// Module: crate::fs
// Provides: {"impl_749"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl fmt :: Debug for Metadata { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut debug = f . debug_struct ("Metadata") ; debug . field ("file_type" , & self . file_type ()) ; debug . field ("permissions" , & self . permissions ()) ; debug . field ("len" , & self . len ()) ; if let Ok (modified) = self . modified () { debug . field ("modified" , & modified) ; } if let Ok (accessed) = self . accessed () { debug . field ("accessed" , & accessed) ; } if let Ok (created) = self . created () { debug . field ("created" , & created) ; } debug . finish_non_exhaustive () } }
};
}

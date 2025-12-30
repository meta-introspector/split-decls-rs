// Generated macro for impl_128 (impl)
macro_rules! Depcrate_errorimpl_128 {
() => {
// Module: crate::error
// Provides: {"impl_128"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I : fmt :: Display > fmt :: Display for TreeErrorBase < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (cause) = self . cause . as_ref () { write ! (f , "caused by {cause}") ? ; } let input = abbreviate (self . input . to_string ()) ; write ! (f , " at '{input}'") ? ; Ok (()) } }
};
}

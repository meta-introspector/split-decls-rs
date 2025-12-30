// Generated macro for impl_129 (impl)
macro_rules! Depcrate_errorimpl_129 {
() => {
// Module: crate::error
// Provides: {"impl_129"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I : fmt :: Display , C : fmt :: Display > fmt :: Display for TreeErrorContext < I , C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let context = & self . context ; let input = abbreviate (self . input . to_string ()) ; write ! (f , "{context} at '{input}'") ? ; Ok (()) } }
};
}

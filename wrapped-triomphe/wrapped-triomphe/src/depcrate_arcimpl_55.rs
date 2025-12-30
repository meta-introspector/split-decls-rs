// Generated macro for impl_55 (impl)
macro_rules! Depcrate_arcimpl_55 {
() => {
// Module: crate::arc
// Provides: {"impl_55"}
// Dependencies: {}
impl < T : ? Sized > fmt :: Pointer for Arc < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Pointer :: fmt (& self . ptr () , f) } }
};
}

// Generated macro for impl_17 (impl)
macro_rules! Depcrate_versionimpl_17 {
() => {
// Module: crate::version
// Provides: {"impl_17"}
// Dependencies: {}
impl fmt :: Display for Version { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let (major , minor , patch) = self . to_mmp () ; write ! (f , "{}.{}.{}" , major , minor , patch) } }
};
}

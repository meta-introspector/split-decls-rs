// Generated macro for impl_88 (impl)
macro_rules! Depcrate_fileimpl_88 {
() => {
// Module: crate::file
// Provides: {"impl_88"}
// Dependencies: {}
impl < F > fmt :: Display for PersistError < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed to persist temporary file: {}" , self . error) } }
};
}

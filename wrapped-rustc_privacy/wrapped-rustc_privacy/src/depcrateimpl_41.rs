// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'tcx > fmt :: Display for LazyDefPathStr < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . tcx . def_path_str (self . def_id)) } }
};
}

// Generated macro for impl_43 (impl)
macro_rules! Depcrate_layoutimpl_43 {
() => {
// Module: crate::layout
// Provides: {"impl_43"}
// Dependencies: {}
impl < R , T > fmt :: Display for Reference < R , T > where R : Region , T : Type , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("&") ? ; if self . is_mut { f . write_str ("mut ") ? ; } self . referent . fmt (f) } }
};
}

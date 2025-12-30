// Generated macro for DEF_ID_DEBUG (static)
macro_rules! Depcrate_def_idDEF_ID_DEBUG {
() => {
// Module: crate::def_id
// Provides: {"DEF_ID_DEBUG"}
// Dependencies: {}
pub static DEF_ID_DEBUG : AtomicRef < fn (DefId , & mut fmt :: Formatter < '_ >) -> fmt :: Result > = AtomicRef :: new (& (default_def_id_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ;
};
}

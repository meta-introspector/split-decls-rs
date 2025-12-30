// Generated macro for default_def_id_debug (function)
macro_rules! Depcrate_def_iddefault_def_id_debug {
() => {
// Module: crate::def_id
// Provides: {"default_def_id_debug"}
// Dependencies: {}
pub fn default_def_id_debug (def_id : DefId , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DefId") . field ("krate" , & def_id . krate) . field ("index" , & def_id . index) . finish () }
};
}

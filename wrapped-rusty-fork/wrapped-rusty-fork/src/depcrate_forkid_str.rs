// Generated macro for id_str (function)
macro_rules! Depcrate_forkid_str {
() => {
// Module: crate::fork
// Provides: {"id_str"}
// Dependencies: {}
fn id_str < ID : Hash > (id : ID) -> String { let mut hasher = fnv :: FnvHasher :: default () ; id . hash (& mut hasher) ; return format ! (":{:016X}" , hasher . finish ()) ; }
};
}

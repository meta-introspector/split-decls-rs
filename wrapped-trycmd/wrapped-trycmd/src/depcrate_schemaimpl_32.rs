// Generated macro for impl_32 (impl)
macro_rules! Depcrate_schemaimpl_32 {
() => {
// Module: crate::schema
// Provides: {"impl_32"}
// Dependencies: {}
impl Filesystem { pub (crate) fn sandbox (& self) -> bool { self . sandbox . unwrap_or_default () } pub (crate) fn rel_cwd (& self) -> Result < & std :: path :: Path , crate :: Error > { if let (Some (orig_cwd) , Some (orig_base)) = (self . cwd . as_deref () , self . base . as_deref ()) { let rel_cwd = orig_cwd . strip_prefix (orig_base) . map_err (| _ | { crate :: Error :: new (format ! ("fs.cwd ({}) must be within fs.base ({})" , orig_cwd . display () , orig_base . display ())) }) ? ; Ok (rel_cwd) } else { Ok (std :: path :: Path :: new ("")) } } }
};
}

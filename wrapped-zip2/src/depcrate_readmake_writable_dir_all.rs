// Generated macro for make_writable_dir_all (function)
macro_rules! Depcrate_readmake_writable_dir_all {
() => {
// Module: crate::read
// Provides: {"make_writable_dir_all"}
// Dependencies: {}
pub (crate) fn make_writable_dir_all < T : AsRef < Path > > (outpath : T) -> Result < () , ZipError > { create_dir_all (outpath . as_ref ()) ? ; # [cfg (unix)] { use std :: os :: unix :: fs :: PermissionsExt ; std :: fs :: set_permissions (outpath . as_ref () , std :: fs :: Permissions :: from_mode (0o700 | std :: fs :: metadata (outpath . as_ref ()) ? . permissions () . mode () ,) ,) ? ; } Ok (()) }
};
}

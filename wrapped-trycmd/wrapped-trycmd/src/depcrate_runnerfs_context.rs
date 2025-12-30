// Generated macro for fs_context (function)
macro_rules! Depcrate_runnerfs_context {
() => {
// Module: crate::runner
// Provides: {"fs_context"}
// Dependencies: {}
# [cfg_attr (not (feature = "filesystem") , allow (unused_variables))] fn fs_context (path : & std :: path :: Path , cwd : Option < & std :: path :: Path > , sandbox : bool , mode : & Mode ,) -> Result < snapbox :: dir :: DirRoot , crate :: Error > { if sandbox { # [cfg (feature = "filesystem")] match mode { Mode :: Dump (root) => { let target = root . join (path . with_extension ("out") . file_name () . unwrap ()) ; let mut context = snapbox :: dir :: DirRoot :: mutable_at (& target) ? ; if let Some (cwd) = cwd { context = context . with_template (cwd) ? ; } Ok (context) } Mode :: Fail | Mode :: Overwrite => { let mut context = snapbox :: dir :: DirRoot :: mutable_temp () ? ; if let Some (cwd) = cwd { context = context . with_template (cwd) ? ; } Ok (context) } } # [cfg (not (feature = "filesystem"))] Err ("Sandboxing is disabled" . into ()) } else { Ok (cwd . map (snapbox :: dir :: DirRoot :: immutable) . unwrap_or_else (snapbox :: dir :: DirRoot :: none)) } }
};
}

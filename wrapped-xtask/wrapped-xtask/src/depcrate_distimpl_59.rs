// Generated macro for impl_59 (impl)
macro_rules! Depcrate_distimpl_59 {
() => {
// Module: crate::dist
// Provides: {"impl_59"}
// Dependencies: {}
impl flags :: Dist { pub (crate) fn run (self , sh : & Shell) -> anyhow :: Result < () > { let stable = sh . var ("GITHUB_REF") . unwrap_or_default () . as_str () == "refs/heads/release" ; let project_root = project_root () ; let target = Target :: get (& project_root , sh) ; let allocator = self . allocator () ; let dist = project_root . join ("dist") ; sh . remove_path (& dist) ? ; sh . create_dir (& dist) ? ; if let Some (patch_version) = self . client_patch_version { let version = if stable { format ! ("{VERSION_STABLE}.{patch_version}") } else { format ! ("{VERSION_NIGHTLY}.{patch_version}") } ; dist_server (sh , & format ! ("{version}-standalone") , & target , allocator , self . zig , self . pgo , self . enable_profiling ,) ? ; let release_tag = if stable { date_iso (sh) ? } else { "nightly" . to_owned () } ; dist_client (sh , & version , & release_tag , & target) ? ; } else { dist_server (sh , "0.0.0-standalone" , & target , allocator , self . zig , self . pgo , self . enable_profiling ,) ? ; } Ok (()) } }
};
}

// Generated macro for dist_server (function)
macro_rules! Depcrate_distdist_server {
() => {
// Module: crate::dist
// Provides: {"dist_server"}
// Dependencies: {}
fn dist_server (sh : & Shell , release : & str , target : & Target , allocator : Malloc , zig : bool , pgo : Option < PgoTrainingCrate > , dev_rel : bool ,) -> anyhow :: Result < () > { let _e = sh . push_env ("CFG_RELEASE" , release) ; let _e = sh . push_env ("CARGO_PROFILE_RELEASE_LTO" , "thin") ; let _e = sh . push_env ("CARGO_PROFILE_DEV_REL_LTO" , "thin") ; let linux_target = target . is_linux () ; let target_name = match & target . libc_suffix { Some (libc_suffix) if zig => format ! ("{}.{libc_suffix}" , target . name) , _ => target . name . to_owned () , } ; let features = allocator . to_features () ; let command = if linux_target && zig { "zigbuild" } else { "build" } ; let pgo_profile = if let Some (train_crate) = pgo { Some (crate :: pgo :: gather_pgo_profile (sh , crate :: pgo :: build_command (sh , command , & target_name , features) , & target_name , train_crate ,) ?) } else { None } ; let mut cmd = build_command (sh , command , & target_name , features , dev_rel) ; if let Some (profile) = pgo_profile { cmd = cmd . env ("RUSTFLAGS" , format ! ("-Cprofile-use={}" , profile . to_str () . unwrap ())) ; } cmd . run () . context ("cannot build Rust Analyzer") ? ; let dst = Path :: new ("dist") . join (& target . artifact_name) ; if target_name . contains ("-windows-") { zip (& target . server_path , target . symbols_path . as_ref () , & dst . with_extension ("zip")) ? ; } else { gzip (& target . server_path , & dst . with_extension ("gz")) ? ; } Ok (()) }
};
}

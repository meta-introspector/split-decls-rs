// Generated macro for macro_238 (macro)
macro_rules! Depcrate_commonmacro_238 {
() => {
// Module: crate::common
// Provides: {"macro_238"}
// Dependencies: {}
cfg_if ! { if # [cfg (all (not (feature = "unknown-ci") , any (target_os = "freebsd" , target_os = "linux" , target_os = "android" , target_os = "macos" , target_os = "ios" ,)))] { uid ! (libc :: uid_t , std :: str :: FromStr) ; gid ! (libc :: gid_t) ; } else if # [cfg (windows)] { uid ! (crate :: windows :: Sid) ; gid ! (u32) ; # [cfg (any (feature = "system" , feature = "user"))] impl std :: str :: FromStr for Uid { type Err = < crate :: windows :: Sid as std :: str :: FromStr >:: Err ; fn from_str (t : & str) -> Result < Self , Self :: Err > { Ok (Self (t . parse () ?)) } } } else { uid ! (u32 , std :: str :: FromStr) ; gid ! (u32) ; } }
};
}

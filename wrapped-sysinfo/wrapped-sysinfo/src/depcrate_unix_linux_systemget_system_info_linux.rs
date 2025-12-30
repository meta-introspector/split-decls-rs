// Generated macro for get_system_info_linux (function)
macro_rules! Depcrate_unix_linux_systemget_system_info_linux {
() => {
// Module: crate::unix::linux::system
// Provides: {"get_system_info_linux"}
// Dependencies: {}
# [cfg (not (target_os = "android"))] fn get_system_info_linux (info : InfoType , path : & Path , fallback_path : & Path) -> Option < String > { if let Ok (buf) = File :: open (path) . and_then (| mut f | { let mut buf = String :: new () ; f . read_to_string (& mut buf) ? ; Ok (buf) }) { let info_str = match info { InfoType :: Name => "NAME=" , InfoType :: OsVersion => "VERSION_ID=" , InfoType :: DistributionID => "ID=" , InfoType :: DistributionIDLike => "ID_LIKE=" , } ; for line in buf . lines () { if let Some (stripped) = line . strip_prefix (info_str) { return Some (stripped . replace ('"' , "")) ; } } } let buf = File :: open (fallback_path) . and_then (| mut f | { let mut buf = String :: new () ; f . read_to_string (& mut buf) ? ; Ok (buf) }) . ok () ? ; let info_str = match info { InfoType :: OsVersion => "DISTRIB_RELEASE=" , InfoType :: Name => "DISTRIB_ID=" , InfoType :: DistributionID => { return None ; } InfoType :: DistributionIDLike => { return None ; } } ; for line in buf . lines () { if let Some (stripped) = line . strip_prefix (info_str) { return Some (stripped . replace ('"' , "")) ; } } None }
};
}

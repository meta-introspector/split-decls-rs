// Generated macro for get_physical_core_count (function)
macro_rules! Depcrate_unix_linux_cpuget_physical_core_count {
() => {
// Module: crate::unix::linux::cpu
// Provides: {"get_physical_core_count"}
// Dependencies: {}
# [allow (unused_assignments)] pub (crate) fn get_physical_core_count () -> Option < usize > { let mut s = String :: new () ; if let Err (_e) = File :: open ("/proc/cpuinfo") . and_then (| mut f | f . read_to_string (& mut s)) { sysinfo_debug ! ("Cannot read `/proc/cpuinfo` file: {:?}" , _e) ; return None ; } macro_rules ! add_core { ($ core_ids_and_physical_ids : ident , $ core_id : ident , $ physical_id : ident , $ cpu : ident) => { { if !$ core_id . is_empty () && !$ physical_id . is_empty () { $ core_ids_and_physical_ids . insert (format ! ("{} {}" , $ core_id , $ physical_id)) ; } else if !$ cpu . is_empty () { $ core_ids_and_physical_ids . insert ($ cpu . to_owned ()) ; } $ core_id = "" ; $ physical_id = "" ; $ cpu = "" ; } } ; } let mut core_ids_and_physical_ids : HashSet < String > = HashSet :: new () ; let mut core_id = "" ; let mut physical_id = "" ; let mut cpu = "" ; for line in s . lines () { if line . is_empty () { add_core ! (core_ids_and_physical_ids , core_id , physical_id , cpu) ; } else if line . starts_with ("processor") { cpu = line . splitn (2 , ':') . last () . map (| x | x . trim ()) . unwrap_or_default () ; } else if line . starts_with ("core id") { core_id = line . splitn (2 , ':') . last () . map (| x | x . trim ()) . unwrap_or_default () ; } else if line . starts_with ("physical id") { physical_id = line . splitn (2 , ':') . last () . map (| x | x . trim ()) . unwrap_or_default () ; } } add_core ! (core_ids_and_physical_ids , core_id , physical_id , cpu) ; Some (core_ids_and_physical_ids . len ()) }
};
}

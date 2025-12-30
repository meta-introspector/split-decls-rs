// Generated macro for update_time_and_memory (function)
macro_rules! Depcrate_unix_linux_processupdate_time_and_memory {
() => {
// Module: crate::unix::linux::process
// Provides: {"update_time_and_memory"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] fn update_time_and_memory (path : & mut PathHandler , entry : & mut ProcessInner , str_parts : & [& str] , uptime : u64 , info : & SystemInfo , refresh_kind : ProcessRefreshKind ,) { { # [allow (clippy :: collapsible_if)] if refresh_kind . memory () { if ! get_memory (path . replace_and_join ("statm") , entry , info) { old_get_memory (entry , str_parts , info) ; } } set_time (entry , u64 :: from_str (str_parts [ProcIndex :: UserTime as usize]) . unwrap_or (0) , u64 :: from_str (str_parts [ProcIndex :: SystemTime as usize]) . unwrap_or (0) ,) ; entry . run_time = uptime . saturating_sub (entry . start_time_without_boot_time) ; } }
};
}

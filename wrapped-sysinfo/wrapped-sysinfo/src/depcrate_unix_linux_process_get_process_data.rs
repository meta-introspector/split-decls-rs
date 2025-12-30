// Generated macro for _get_process_data (function)
macro_rules! Depcrate_unix_linux_process_get_process_data {
() => {
// Module: crate::unix::linux::process
// Provides: {"_get_process_data"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] pub (crate) fn _get_process_data (path : & Path , proc_list : & mut HashMap < Pid , Process > , pid : Pid , parent_pid : Option < Pid > , uptime : u64 , info : & SystemInfo , refresh_kind : ProcessRefreshKind , tasks : Option < HashSet < Pid > > ,) -> Result < Option < Process > , () > { if let Some (ref mut entry) = proc_list . get_mut (& pid) { return update_existing_process (entry , parent_pid , uptime , info , refresh_kind , tasks) ; } let mut stat_file = None ; let data = _get_stat_data (path , & mut stat_file) ? ; let parts = parse_stat_file (& data) . ok_or (()) ? ; let mut new_process = retrieve_all_new_process_info (pid , parent_pid , & parts , path , info , refresh_kind , uptime) ; new_process . inner . stat_file = stat_file ; new_process . inner . tasks = tasks ; Ok (Some (new_process)) }
};
}

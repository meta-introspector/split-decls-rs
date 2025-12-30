// Generated macro for update_existing_process (function)
macro_rules! Depcrate_unix_linux_processupdate_existing_process {
() => {
// Module: crate::unix::linux::process
// Provides: {"update_existing_process"}
// Dependencies: {}
fn update_existing_process (proc : & mut Process , parent_pid : Option < Pid > , uptime : u64 , info : & SystemInfo , refresh_kind : ProcessRefreshKind , tasks : Option < HashSet < Pid > > ,) -> Result < Option < Process > , () > { let entry = & mut proc . inner ; let data = if let Some (mut f) = entry . stat_file . take () { match get_all_data_from_file (& mut f , 1024) { Ok (data) => { entry . stat_file = Some (f) ; data } Err (_) => { _get_stat_data (& entry . proc_path , & mut entry . stat_file) ? } } } else { _get_stat_data (& entry . proc_path , & mut entry . stat_file) ? } ; entry . tasks = tasks ; let parts = parse_stat_file (& data) . ok_or (()) ? ; let start_time_raw = start_time_raw (& parts) ; if start_time_raw == entry . start_time_raw { let mut proc_path = PathHandler :: new (& entry . proc_path) ; update_proc_info (entry , parent_pid , refresh_kind , & mut proc_path , & parts . str_parts , uptime , info ,) ; refresh_user_group_ids (entry , & mut proc_path , refresh_kind) ; return Ok (None) ; } let p = retrieve_all_new_process_info (entry . pid , parent_pid , & parts , & entry . proc_path , info , refresh_kind , uptime ,) ; * proc = p ; Ok (None) }
};
}

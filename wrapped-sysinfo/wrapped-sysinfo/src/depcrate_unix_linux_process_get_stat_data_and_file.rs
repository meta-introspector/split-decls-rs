// Generated macro for _get_stat_data_and_file (function)
macro_rules! Depcrate_unix_linux_process_get_stat_data_and_file {
() => {
// Module: crate::unix::linux::process
// Provides: {"_get_stat_data_and_file"}
// Dependencies: {}
fn _get_stat_data_and_file (path : & Path) -> Result < (Vec < u8 > , File) , () > { let mut file = File :: open (path . join ("stat")) . map_err (| _ | ()) ? ; let data = get_all_data_from_file (& mut file , 1024) . map_err (| _ | ()) ? ; Ok ((data , file)) }
};
}

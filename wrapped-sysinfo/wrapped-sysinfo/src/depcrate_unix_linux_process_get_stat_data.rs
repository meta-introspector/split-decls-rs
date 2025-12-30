// Generated macro for _get_stat_data (function)
macro_rules! Depcrate_unix_linux_process_get_stat_data {
() => {
// Module: crate::unix::linux::process
// Provides: {"_get_stat_data"}
// Dependencies: {}
fn _get_stat_data (path : & Path , stat_file : & mut Option < FileCounter >) -> Result < Vec < u8 > , () > { let (data , file) = _get_stat_data_and_file (path) ? ; * stat_file = FileCounter :: new (file) ; Ok (data) }
};
}

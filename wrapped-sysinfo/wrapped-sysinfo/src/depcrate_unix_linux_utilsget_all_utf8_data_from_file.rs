// Generated macro for get_all_utf8_data_from_file (function)
macro_rules! Depcrate_unix_linux_utilsget_all_utf8_data_from_file {
() => {
// Module: crate::unix::linux::utils
// Provides: {"get_all_utf8_data_from_file"}
// Dependencies: {}
# [cfg (any (feature = "disk" , feature = "system"))] pub (crate) fn get_all_utf8_data_from_file (file : & mut File , size : usize) -> io :: Result < String > { let mut buf = String :: with_capacity (size) ; file . rewind () ? ; file . read_to_string (& mut buf) ? ; Ok (buf) }
};
}

// Generated macro for get_all_data_from_file (function)
macro_rules! Depcrate_unix_linux_utilsget_all_data_from_file {
() => {
// Module: crate::unix::linux::utils
// Provides: {"get_all_data_from_file"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) fn get_all_data_from_file (file : & mut File , size : usize) -> io :: Result < Vec < u8 > > { let mut buf = Vec :: with_capacity (size) ; file . rewind () ? ; file . read_to_end (& mut buf) ? ; Ok (buf) }
};
}

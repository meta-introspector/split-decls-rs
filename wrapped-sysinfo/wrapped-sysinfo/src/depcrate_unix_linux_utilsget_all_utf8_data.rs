// Generated macro for get_all_utf8_data (function)
macro_rules! Depcrate_unix_linux_utilsget_all_utf8_data {
() => {
// Module: crate::unix::linux::utils
// Provides: {"get_all_utf8_data"}
// Dependencies: {}
# [cfg (any (feature = "disk" , feature = "system"))] pub (crate) fn get_all_utf8_data < P : AsRef < Path > > (file_path : P , size : usize) -> io :: Result < String > { let mut file = File :: open (file_path . as_ref ()) ? ; get_all_utf8_data_from_file (& mut file , size) }
};
}

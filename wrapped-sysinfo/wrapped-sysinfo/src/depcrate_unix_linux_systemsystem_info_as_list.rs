// Generated macro for system_info_as_list (function)
macro_rules! Depcrate_unix_linux_systemsystem_info_as_list {
() => {
// Module: crate::unix::linux::system
// Provides: {"system_info_as_list"}
// Dependencies: {}
# [doc = " Returns a system info value as a list of strings."] # [doc = " Absence of a value is treated as an empty list."] fn system_info_as_list (sysinfo : Option < String >) -> Vec < String > { match sysinfo { Some (value) => value . split_ascii_whitespace () . map (String :: from) . collect () , None => Vec :: new () , } }
};
}

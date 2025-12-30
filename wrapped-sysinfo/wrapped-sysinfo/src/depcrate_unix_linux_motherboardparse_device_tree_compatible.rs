// Generated macro for parse_device_tree_compatible (function)
macro_rules! Depcrate_unix_linux_motherboardparse_device_tree_compatible {
() => {
// Module: crate::unix::linux::motherboard
// Provides: {"parse_device_tree_compatible"}
// Dependencies: {}
fn parse_device_tree_compatible () -> Option < (String , String) > { let bytes = read ("/proc/device-tree/compatible") . ok () ? ; let first_line = bytes . split (| & b | b == 0) . next () ? ; std :: str :: from_utf8 (first_line) . ok () ? . split_once (',') . map (| (a , b) | (a . to_owned () , b . to_owned ())) }
};
}

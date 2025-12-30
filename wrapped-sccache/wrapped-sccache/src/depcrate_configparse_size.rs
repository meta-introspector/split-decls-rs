// Generated macro for parse_size (function)
macro_rules! Depcrate_configparse_size {
() => {
// Module: crate::config
// Provides: {"parse_size"}
// Dependencies: {}
pub fn parse_size (val : & str) -> Option < u64 > { let multiplier = match val . chars () . last () . map (| v | v . to_ascii_uppercase ()) { Some ('K') => 1024 , Some ('M') => 1024 * 1024 , Some ('G') => 1024 * 1024 * 1024 , Some ('T') => 1024 * 1024 * 1024 * 1024 , _ => 1 , } ; let val = if multiplier > 1 && ! val . is_empty () { val . split_at (val . len () - 1) . 0 } else { val } ; u64 :: from_str (val) . ok () . map (| size | size * multiplier) }
};
}

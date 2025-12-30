// Generated macro for parse_stat_file (function)
macro_rules! Depcrate_unix_linux_processparse_stat_file {
() => {
// Module: crate::unix::linux::process
// Provides: {"parse_stat_file"}
// Dependencies: {}
fn parse_stat_file (data : & [u8]) -> Option < Parts < '_ > > { let mut str_parts = Vec :: with_capacity (51) ; let mut data_it = data . splitn (2 , | & b | b == b' ') ; str_parts . push (str :: from_utf8 (data_it . next () ?) . ok () ?) ; let mut data_it = data_it . next () ? . rsplitn (2 , | & b | b == b')') ; let data = str :: from_utf8 (data_it . next () ?) . ok () ? ; let short_exe = data_it . next () ? ; str_parts . extend (data . split_whitespace ()) ; Some (Parts { str_parts , short_exe : short_exe . strip_prefix (b"(") . unwrap_or (short_exe) , }) }
};
}

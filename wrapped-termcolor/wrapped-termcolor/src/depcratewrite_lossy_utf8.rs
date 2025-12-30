// Generated macro for write_lossy_utf8 (function)
macro_rules! Depcratewrite_lossy_utf8 {
() => {
// Module: crate
// Provides: {"write_lossy_utf8"}
// Dependencies: {}
# [cfg (windows)] fn write_lossy_utf8 < W : io :: Write > (mut w : W , buf : & [u8]) -> io :: Result < usize > { match :: std :: str :: from_utf8 (buf) { Ok (s) => w . write (s . as_bytes ()) , Err (ref e) if e . valid_up_to () == 0 => { w . write (b"\xEF\xBF\xBD") ? ; Ok (1) } Err (e) => w . write (& buf [.. e . valid_up_to ()]) , } }
};
}

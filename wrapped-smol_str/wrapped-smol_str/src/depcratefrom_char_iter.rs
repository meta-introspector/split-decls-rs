// Generated macro for from_char_iter (function)
macro_rules! Depcratefrom_char_iter {
() => {
// Module: crate
// Provides: {"from_char_iter"}
// Dependencies: {}
# [inline] fn from_char_iter (iter : impl Iterator < Item = char >) -> SmolStr { from_buf_and_chars ([0 ; _] , 0 , iter) }
};
}

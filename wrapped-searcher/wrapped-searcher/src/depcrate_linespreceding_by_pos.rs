// Generated macro for preceding_by_pos (function)
macro_rules! Depcrate_linespreceding_by_pos {
() => {
// Module: crate::lines
// Provides: {"preceding_by_pos"}
// Dependencies: {}
# [doc = " Returns the minimal starting offset of the line that occurs `count` lines"] # [doc = " before the line containing `pos`. Lines are terminated by `line_term`."] # [doc = " If `count` is zero, then this returns the starting offset of the line"] # [doc = " containing `pos`."] # [doc = ""] # [doc = " If `pos` points just past a line terminator, then it is considered part of"] # [doc = " the line that it terminates. For example, given `bytes = b\"abc\\nxyz\\n\"`"] # [doc = " and `pos = 7`, `preceding(bytes, pos, b'\\n', 0)` returns `4` (as does `pos"] # [doc = " = 8`) and `preceding(bytes, pos, `b'\\n', 1)` returns `0`."] fn preceding_by_pos (bytes : & [u8] , mut pos : usize , line_term : u8 , mut count : usize ,) -> usize { if pos == 0 { return 0 ; } else if bytes [pos - 1] == line_term { pos -= 1 ; } loop { match bytes [.. pos] . rfind_byte (line_term) { None => { return 0 ; } Some (i) => { if count == 0 { return i + 1 ; } else if i == 0 { return 0 ; } count -= 1 ; pos = i ; } } } }
};
}

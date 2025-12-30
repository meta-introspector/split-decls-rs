// Generated macro for preceding (function)
macro_rules! Depcrate_linespreceding {
() => {
// Module: crate::lines
// Provides: {"preceding"}
// Dependencies: {}
# [doc = " Returns the minimal starting offset of the line that occurs `count` lines"] # [doc = " before the last line in `bytes`."] # [doc = ""] # [doc = " Lines are terminated by `line_term`. If `count` is zero, then this returns"] # [doc = " the starting offset of the last line in `bytes`."] # [doc = ""] # [doc = " If `bytes` ends with a line terminator, then the terminator itself is"] # [doc = " considered part of the last line."] pub (crate) fn preceding (bytes : & [u8] , line_term : u8 , count : usize) -> usize { preceding_by_pos (bytes , bytes . len () , line_term , count) }
};
}

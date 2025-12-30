// Generated macro for without_terminator (function)
macro_rules! Depcrate_lineswithout_terminator {
() => {
// Module: crate::lines
// Provides: {"without_terminator"}
// Dependencies: {}
# [doc = " Given a line that possibly ends with a terminator, return that line without"] # [doc = " the terminator."] # [inline (always)] pub (crate) fn without_terminator (bytes : & [u8] , line_term : LineTerminator ,) -> & [u8] { let line_term = line_term . as_bytes () ; let start = bytes . len () . saturating_sub (line_term . len ()) ; if bytes . get (start ..) == Some (line_term) { return & bytes [.. bytes . len () - line_term . len ()] ; } bytes }
};
}

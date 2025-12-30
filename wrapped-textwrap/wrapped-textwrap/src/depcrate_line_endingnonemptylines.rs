// Generated macro for NonEmptyLines (struct)
macro_rules! Depcrate_line_endingNonEmptyLines {
() => {
// Module: crate::line_ending
// Provides: {"NonEmptyLines"}
// Dependencies: {}
# [doc = " An iterator over the lines of a string, as tuples of string slice"] # [doc = " and [`LineEnding`] value; it only emits non-empty lines (i.e. having"] # [doc = " some content before the terminating `\\r\\n` or `\\n`)."] # [doc = ""] # [doc = " This struct is used internally by the library."] # [derive (Debug , Clone , Copy)] pub (crate) struct NonEmptyLines < 'a > (pub & 'a str) ;
};
}

// Generated macro for impl_134 (impl)
macro_rules! Depcrate_cursorimpl_134 {
() => {
// Module: crate::cursor
// Provides: {"impl_134"}
// Dependencies: {}
impl fmt :: Write for ZeroAsciiIgnoreCaseTrieCursor < '_ > { # [doc = " Steps the cursor through each ASCII byte of the string."] # [doc = ""] # [doc = " If the string contains non-ASCII chars, an error is returned."] fn write_str (& mut self , s : & str) -> fmt :: Result { for b in s . bytes () { if ! b . is_ascii () { return Err (fmt :: Error) ; } self . step (b) ; } Ok (()) } # [doc = " Equivalent to [`ZeroAsciiIgnoreCaseTrieCursor::step()`], except returns"] # [doc = " an error if the char is non-ASCII."] fn write_char (& mut self , c : char) -> fmt :: Result { if ! c . is_ascii () { return Err (fmt :: Error) ; } self . step (c as u8) ; Ok (()) } }
};
}

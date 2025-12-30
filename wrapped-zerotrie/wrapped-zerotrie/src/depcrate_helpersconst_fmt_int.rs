// Generated macro for const_fmt_int (function)
macro_rules! Depcrate_helpersconst_fmt_int {
() => {
// Module: crate::helpers
// Provides: {"const_fmt_int"}
// Dependencies: {}
# [doc = " Formats a usize as a string of length N, padded with spaces,"] # [doc = " with the given prefix."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the string is too short, the function may panic. To prevent"] # [doc = " this, N should be MAX_USIZE_LEN_AS_DIGITS larger than M."] # [allow (clippy :: indexing_slicing)] pub (crate) const fn const_fmt_int < const M : usize , const N : usize > (prefix : [u8 ; M] , value : usize ,) -> [u8 ; N] { let mut output = [b' ' ; N] ; let mut i = 0 ; while i < M { output [i] = prefix [i] ; i += 1 ; } let mut int_only = [b' ' ; MAX_USIZE_LEN_AS_DIGITS] ; let mut value = value ; let mut i = MAX_USIZE_LEN_AS_DIGITS - 1 ; loop { let x = (value % 10) as u8 ; int_only [i] = x + b'0' ; value /= 10 ; if value == 0 { break ; } i -= 1 ; } let mut j = M ; while i < MAX_USIZE_LEN_AS_DIGITS { output [j] = int_only [i] ; j += 1 ; i += 1 ; } output }
};
}

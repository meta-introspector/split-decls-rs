// Generated macro for keccak_function (macro)
macro_rules! Depcratekeccak_function {
() => {
// Module: crate
// Provides: {"keccak_function"}
// Dependencies: {}
macro_rules ! keccak_function { ($ doc : expr , $ name : ident , $ rounds : expr , $ rc : expr) => { # [doc = $ doc] # [allow (unused_assignments)] # [allow (non_upper_case_globals)] pub fn $ name (a : & mut [u64 ; $ crate :: WORDS]) { use crunchy :: unroll ; for i in 0 ..$ rounds { let mut array : [u64 ; 5] = [0 ; 5] ; unroll ! { for x in 0 .. 5 { unroll ! { for y_count in 0 .. 5 { let y = y_count * 5 ; array [x] ^= a [x + y] ; } } } } unroll ! { for x in 0 .. 5 { unroll ! { for y_count in 0 .. 5 { let y = y_count * 5 ; a [y + x] ^= array [(x + 4) % 5] ^ array [(x + 1) % 5] . rotate_left (1) ; } } } } let mut last = a [1] ; unroll ! { for x in 0 .. 24 { array [0] = a [$ crate :: PI [x]] ; a [$ crate :: PI [x]] = last . rotate_left ($ crate :: RHO [x]) ; last = array [0] ; } } unroll ! { for y_step in 0 .. 5 { let y = y_step * 5 ; unroll ! { for x in 0 .. 5 { array [x] = a [y + x] ; } } unroll ! { for x in 0 .. 5 { a [y + x] = array [x] ^ ((! array [(x + 1) % 5]) & (array [(x + 2) % 5])) ; } } } } ; a [0] ^= $ rc [i] ; } } } }
};
}

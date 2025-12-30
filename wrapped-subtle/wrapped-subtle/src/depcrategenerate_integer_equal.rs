// Generated macro for generate_integer_equal (macro)
macro_rules! Depcrategenerate_integer_equal {
() => {
// Module: crate
// Provides: {"generate_integer_equal"}
// Dependencies: {}
# [doc = " Given the bit-width `$bit_width` and the corresponding primitive"] # [doc = " unsigned and signed types `$t_u` and `$t_i` respectively, generate"] # [doc = " an `ConstantTimeEq` implementation."] macro_rules ! generate_integer_equal { ($ t_u : ty , $ t_i : ty , $ bit_width : expr) => { impl ConstantTimeEq for $ t_u { # [inline] fn ct_eq (& self , other : &$ t_u) -> Choice { let x : $ t_u = self ^ other ; let y : $ t_u = (x | x . wrapping_neg ()) >> ($ bit_width - 1) ; ((y ^ (1 as $ t_u)) as u8) . into () } } impl ConstantTimeEq for $ t_i { # [inline] fn ct_eq (& self , other : &$ t_i) -> Choice { (* self as $ t_u) . ct_eq (& (* other as $ t_u)) } } } ; }
};
}

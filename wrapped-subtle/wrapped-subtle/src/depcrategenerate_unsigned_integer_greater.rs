// Generated macro for generate_unsigned_integer_greater (macro)
macro_rules! Depcrategenerate_unsigned_integer_greater {
() => {
// Module: crate
// Provides: {"generate_unsigned_integer_greater"}
// Dependencies: {}
macro_rules ! generate_unsigned_integer_greater { ($ t_u : ty , $ bit_width : expr) => { impl ConstantTimeGreater for $ t_u { # [doc = " Returns Choice::from(1) iff x > y, and Choice::from(0) iff x <= y."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This algoritm would also work for signed integers if we first"] # [doc = " flip the top bit, e.g. `let x: u8 = x ^ 0x80`, etc."] # [inline] fn ct_gt (& self , other : &$ t_u) -> Choice { let gtb = self & ! other ; let mut ltb = ! self & other ; let mut pow = 1 ; while pow < $ bit_width { ltb |= ltb >> pow ; pow += pow ; } let mut bit = gtb & ! ltb ; let mut pow = 1 ; while pow < $ bit_width { bit |= bit >> pow ; pow += pow ; } Choice :: from ((bit & 1) as u8) } } } ; }
};
}

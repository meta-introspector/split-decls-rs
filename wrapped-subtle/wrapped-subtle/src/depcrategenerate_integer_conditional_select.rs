// Generated macro for generate_integer_conditional_select (macro)
macro_rules! Depcrategenerate_integer_conditional_select {
() => {
// Module: crate
// Provides: {"generate_integer_conditional_select"}
// Dependencies: {}
macro_rules ! generate_integer_conditional_select { ($ ($ t : tt) *) => ($ (impl ConditionallySelectable for $ t { # [inline] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { let mask = - (choice . unwrap_u8 () as to_signed_int ! ($ t)) as $ t ; a ^ (mask & (a ^ b)) } # [inline] fn conditional_assign (& mut self , other : & Self , choice : Choice) { let mask = - (choice . unwrap_u8 () as to_signed_int ! ($ t)) as $ t ; * self ^= mask & (* self ^ * other) ; } # [inline] fn conditional_swap (a : & mut Self , b : & mut Self , choice : Choice) { let mask = - (choice . unwrap_u8 () as to_signed_int ! ($ t)) as $ t ; let t = mask & (* a ^ * b) ; * a ^= t ; * b ^= t ; } }) *) }
};
}

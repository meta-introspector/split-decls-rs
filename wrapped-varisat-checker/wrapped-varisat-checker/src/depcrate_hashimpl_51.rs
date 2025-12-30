// Generated macro for impl_51 (impl)
macro_rules! Depcrate_hashimpl_51 {
() => {
// Module: crate::hash
// Provides: {"impl_51"}
// Dependencies: {}
impl ClauseHasher { # [doc = " Compute a clause hash of the current bit size"] pub fn clause_hash (& self , lits : & [Lit]) -> ClauseHash { let shift_bits = ClauseHash :: max_value () . count_ones () - self . hash_bits ; let mut hash = 0 ; for & lit in lits . iter () { match self . solver_var_names . get (& lit . var ()) { Some (var) => hash ^= lit_hash (var . lit (lit . is_positive ())) , None => hash ^= lit_code_hash (lit . code () + Var :: max_count () * 2) , } } hash >> shift_bits } }
};
}

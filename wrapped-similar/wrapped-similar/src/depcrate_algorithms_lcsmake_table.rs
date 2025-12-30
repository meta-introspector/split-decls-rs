// Generated macro for make_table (function)
macro_rules! Depcrate_algorithms_lcsmake_table {
() => {
// Module: crate::algorithms::lcs
// Provides: {"make_table"}
// Dependencies: {}
fn make_table < Old , New > (old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > , deadline : Option < Instant > ,) -> Option < BTreeMap < (usize , usize) , u32 > > where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , New :: Output : PartialEq < Old :: Output > , { let old_len = old_range . len () ; let new_len = new_range . len () ; let mut table = BTreeMap :: new () ; for i in (0 .. new_len) . rev () { if deadline_exceeded (deadline) { return None ; } for j in (0 .. old_len) . rev () { let val = if new [i] == old [j] { table . get (& (i + 1 , j + 1)) . unwrap_or (& 0) + 1 } else { * table . get (& (i + 1 , j)) . unwrap_or (& 0) . max (table . get (& (i , j + 1)) . unwrap_or (& 0)) } ; if val > 0 { table . insert ((i , j) , val) ; } } } Some (table) }
};
}

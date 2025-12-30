// Generated macro for shuffle (function)
macro_rules! Depcrate_helpers_shuffleshuffle {
() => {
// Module: crate::helpers::shuffle
// Provides: {"shuffle"}
// Dependencies: {}
fn shuffle < T > (rng : & mut Rng , slice : & mut [T]) { for i in 0 .. slice . len () { randomize_first (rng , & mut slice [i ..]) ; } fn randomize_first < T > (rng : & mut Rng , slice : & mut [T]) { assert ! (! slice . is_empty ()) ; let idx = rng . rand_range (0 .. slice . len () as u64) as usize ; slice . swap (0 , idx) ; } }
};
}

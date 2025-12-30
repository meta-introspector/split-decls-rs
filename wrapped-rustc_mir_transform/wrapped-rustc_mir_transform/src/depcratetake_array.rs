// Generated macro for take_array (function)
macro_rules! Depcratetake_array {
() => {
// Module: crate
// Provides: {"take_array"}
// Dependencies: {}
fn take_array < T , const N : usize > (b : & mut Box < [T] >) -> Result < [T ; N] , Box < [T] > > { let b : Box < [T ; N] > = std :: mem :: take (b) . try_into () ? ; Ok (* b) }
};
}

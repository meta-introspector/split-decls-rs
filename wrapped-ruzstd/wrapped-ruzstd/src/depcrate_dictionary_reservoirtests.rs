// Generated macro for tests (module)
macro_rules! Depcrate_dictionary_reservoirtests {
() => {
// Module: crate::dictionary::reservoir
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Reservoir ; use alloc :: vec ; # [test] fn initial_fill () { let r = Reservoir :: new (16) ; let test_data = vec ! [0_u8 ; 16] ; let output = r . fill (& mut test_data . as_slice ()) ; assert_eq ! (test_data , output) ; } # [test] fn shrinks_for_small_sample () { let r = Reservoir :: new (32) ; let test_data = vec ! [0_u8 ; 28] ; let output = r . fill (& mut test_data . as_slice ()) ; assert ! (output . len () == 28) ; } # [test] fn lake_doesnt_grow () { let r = Reservoir :: new (32) ; let test_data = vec ! [0_u8 ; 16_000_000] ; let output = r . fill (& mut test_data . as_slice ()) ; assert ! (output . len () == 32) ; } }
};
}

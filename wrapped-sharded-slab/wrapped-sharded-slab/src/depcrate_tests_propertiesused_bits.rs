// Generated macro for used_bits (function)
macro_rules! Depcrate_tests_propertiesused_bits {
() => {
// Module: crate::tests::properties
// Provides: {"used_bits"}
// Dependencies: {}
fn used_bits < C : Config > (key : usize) -> usize { assert_eq ! (C :: RESERVED_BITS + Slab ::< u32 , C >:: USED_BITS , std :: mem :: size_of ::< usize > () * 8) ; key & ((! 0) >> C :: RESERVED_BITS) }
};
}

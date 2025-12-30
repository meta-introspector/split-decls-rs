// Generated macro for is_part_of_type (function)
macro_rules! Depcrate_stringis_part_of_type {
() => {
// Module: crate::string
// Provides: {"is_part_of_type"}
// Dependencies: {}
fn is_part_of_type (input : & [& str] , pos : usize) -> bool { input . get (pos ..= pos + 1) == Some (& [":" , ":"]) || input . get (pos . saturating_sub (1) ..= pos) == Some (& [":" , ":"]) }
};
}

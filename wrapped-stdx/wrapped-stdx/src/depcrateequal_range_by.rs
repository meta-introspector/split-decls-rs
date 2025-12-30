// Generated macro for equal_range_by (function)
macro_rules! Depcrateequal_range_by {
() => {
// Module: crate
// Provides: {"equal_range_by"}
// Dependencies: {}
pub fn equal_range_by < T , F > (slice : & [T] , mut key : F) -> ops :: Range < usize > where F : FnMut (& T) -> Ordering , { let start = slice . partition_point (| it | key (it) == Ordering :: Less) ; let len = slice [start ..] . partition_point (| it | key (it) == Ordering :: Equal) ; start .. start + len }
};
}

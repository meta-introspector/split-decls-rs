// Generated macro for bsearch_range_table (function)
macro_rules! Depcrate_tablesbsearch_range_table {
() => {
// Module: crate::tables
// Provides: {"bsearch_range_table"}
// Dependencies: {}
fn bsearch_range_table (c : char , r : & [(char , char)]) -> bool { use core :: cmp :: Ordering :: { Equal , Greater , Less } ; r . binary_search_by (| & (lo , hi) | { if lo > c { Greater } else if hi < c { Less } else { Equal } }) . is_ok () }
};
}

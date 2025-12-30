// Generated macro for test (module)
macro_rules! Depcrate_compiler_counted_arraytest {
() => {
// Module: crate::compiler::counted_array
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [test] fn counted_array_macro () { counted_array ! (static ARR_QUAD : [u8 ; _] = [1 , 2 , 3 , 4 ,]) ; assert_eq ! (ARR_QUAD . len () , 4) ; } }
};
}

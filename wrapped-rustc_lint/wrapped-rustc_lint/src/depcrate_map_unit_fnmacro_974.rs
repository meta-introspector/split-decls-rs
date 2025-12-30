// Generated macro for macro_974 (macro)
macro_rules! Depcrate_map_unit_fnmacro_974 {
() => {
// Module: crate::map_unit_fn
// Provides: {"macro_974"}
// Dependencies: {}
declare_lint ! { # [doc = " The `map_unit_fn` lint checks for `Iterator::map` receive"] # [doc = " a callable that returns `()`."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn foo(items: &mut Vec<u8>) {"] # [doc = "     items.sort();"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let mut x: Vec<Vec<u8>> = vec!["] # [doc = "         vec![0, 2, 1],"] # [doc = "         vec![5, 4, 3],"] # [doc = "     ];"] # [doc = "     x.iter_mut().map(foo);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Mapping to `()` is almost always a mistake."] pub MAP_UNIT_FN , Warn , "`Iterator::map` call that discard the iterator's values" }
};
}

// Generated macro for counted_array (macro)
macro_rules! Depcrate_compiler_counted_arraycounted_array {
() => {
// Module: crate::compiler::counted_array
// Provides: {"counted_array"}
// Dependencies: {}
# [doc = " Helper macro to create fixed-length arrays without specifying a fixed size"] # [macro_export] macro_rules ! counted_array { ($ v : vis static $ name : ident : [$ t : ty ; _] = [$ ($ value : expr) ,* $ (,) ?]) => { $ v static $ name : [$ t ; counted_array ! (@ count $ ($ value ,) *)] = [$ ($ value) ,*] ; } ; (@ count) => { 0usize } ; (@ count $ ($ arg : expr ,) *) => { < [()] >:: len (& [$ (counted_array ! (@ nil $ arg) ,) *]) } ; (@ nil $ orig : expr) => { () } ; }
};
}

// Generated macro for UniformTuple (trait)
macro_rules! Depcrate_cnfUniformTuple {
() => {
// Module: crate::cnf
// Provides: {"UniformTuple"}
// Dependencies: {}
# [doc = " Helper trait to initialize multiple variables of the same type."] pub trait UniformTuple < Item > { fn tuple_len () -> usize ; fn tuple_from_iter (items : impl Iterator < Item = Item >) -> Self ; }
};
}

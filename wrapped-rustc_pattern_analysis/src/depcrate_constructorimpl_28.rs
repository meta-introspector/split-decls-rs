// Generated macro for impl_28 (impl)
macro_rules! Depcrate_constructorimpl_28 {
() => {
// Module: crate::constructor
// Provides: {"impl_28"}
// Dependencies: {}
impl SliceKind { pub fn arity (self) -> usize { match self { FixedLen (length) => length , VarLen (prefix , suffix) => prefix + suffix , } } # [doc = " Whether this pattern includes patterns of length `other_len`."] fn covers_length (self , other_len : usize) -> bool { match self { FixedLen (len) => len == other_len , VarLen (prefix , suffix) => prefix + suffix <= other_len , } } }
};
}

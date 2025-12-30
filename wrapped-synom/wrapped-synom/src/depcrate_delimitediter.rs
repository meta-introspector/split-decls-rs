// Generated macro for Iter (struct)
macro_rules! Depcrate_delimitedIter {
() => {
// Module: crate::delimited
// Provides: {"Iter"}
// Dependencies: {}
pub struct Iter < 'a , T : 'a , D : 'a > { inner : slice :: Iter < 'a , (T , Option < D >) > , }
};
}

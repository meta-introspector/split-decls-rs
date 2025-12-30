// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a , T , const N : usize > Iterator for Iter < 'a , T , N > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { let entry = self . cache . entries . get (self . pos as usize) ? ; self . pos = if self . pos == self . cache . tail { N as u16 } else { entry . next } ; Some (& entry . val) } }
};
}

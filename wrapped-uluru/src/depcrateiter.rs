// Generated macro for Iter (struct)
macro_rules! DepcrateIter {
() => {
// Module: crate
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Iterator over values in an [`LRUCache`], from most-recently-used to least-recently-used."] pub struct Iter < 'a , T , const N : usize > { cache : & 'a LRUCache < T , N > , pos : u16 , }
};
}

// Generated macro for IterMut (struct)
macro_rules! DepcrateIterMut {
() => {
// Module: crate
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " Mutable iterator over values in an `LRUCache`, from most-recently-used to least-recently-used."] struct IterMut < 'a , T , const N : usize > { cache : & 'a mut LRUCache < T , N > , pos : u16 , }
};
}

// Generated macro for items (function)
macro_rules! Depcrate_testsitems {
() => {
// Module: crate::tests
// Provides: {"items"}
// Dependencies: {}
# [doc = " Convenience function for test assertions"] fn items < T , const N : usize > (cache : & mut LRUCache < T , N >) -> Vec < T > where T : Clone , { let mut v = Vec :: new () ; let mut iter = cache . iter_mut () ; while let Some ((_idx , val)) = iter . next () { v . push (val . clone ()) } v }
};
}

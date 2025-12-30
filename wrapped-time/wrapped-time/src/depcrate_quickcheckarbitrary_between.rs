// Generated macro for arbitrary_between (macro)
macro_rules! Depcrate_quickcheckarbitrary_between {
() => {
// Module: crate::quickcheck
// Provides: {"arbitrary_between"}
// Dependencies: {}
# [doc = " Obtain an arbitrary value between the minimum and maximum inclusive."] macro_rules ! arbitrary_between { ($ type : ty ; $ gen : expr , $ min : expr , $ max : expr) => { { let min = $ min ; let max = $ max ; let range = max - min ; <$ type >:: arbitrary ($ gen) . rem_euclid (range + 1) + min } } ; }
};
}

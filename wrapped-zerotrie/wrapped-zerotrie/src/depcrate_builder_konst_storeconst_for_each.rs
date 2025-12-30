// Generated macro for const_for_each (macro)
macro_rules! Depcrate_builder_konst_storeconst_for_each {
() => {
// Module: crate::builder::konst::store
// Provides: {"const_for_each"}
// Dependencies: {}
# [doc = " Evaluates a block over each element of a const slice. Takes three arguments:"] # [doc = ""] # [doc = " 1. Expression that resolves to the [`ConstSlice`]."] # [doc = " 2. Token that will be assigned the value of the element."] # [doc = " 3. Block to evaluate for each element."] macro_rules ! const_for_each { ($ safe_const_slice : expr , $ item : tt , $ inner : expr) => { { let mut i = 0 ; while i < $ safe_const_slice . len () { let $ item = $ safe_const_slice . get_or_panic (i) ; $ inner ; i += 1 ; } } } ; }
};
}

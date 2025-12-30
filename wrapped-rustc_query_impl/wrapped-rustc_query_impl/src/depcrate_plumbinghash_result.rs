// Generated macro for hash_result (macro)
macro_rules! Depcrate_plumbinghash_result {
() => {
// Module: crate::plumbing
// Provides: {"hash_result"}
// Dependencies: {}
macro_rules ! hash_result { ([] [$ V : ty]) => { { Some (| hcx , result | dep_graph :: hash_result (hcx , & restore ::<$ V > (* result))) } } ; ([(no_hash) $ ($ rest : tt) *] [$ V : ty]) => { { None } } ; ([$ other : tt $ ($ modifiers : tt) *] [$ ($ args : tt) *]) => { hash_result ! ([$ ($ modifiers) *] [$ ($ args) *]) } ; }
};
}

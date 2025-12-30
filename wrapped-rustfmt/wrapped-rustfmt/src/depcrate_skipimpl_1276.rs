// Generated macro for impl_1276 (impl)
macro_rules! Depcrate_skipimpl_1276 {
() => {
// Module: crate::skip
// Provides: {"impl_1276"}
// Dependencies: {}
impl SkipContext { pub (crate) fn update_with_attrs (& mut self , attrs : & [ast :: Attribute]) { self . macros . extend (get_skip_names ("macros" , attrs)) ; self . attributes . extend (get_skip_names ("attributes" , attrs)) ; } pub (crate) fn update (& mut self , other : SkipContext) { let SkipContext { macros , attributes } = other ; self . macros . update (macros) ; self . attributes . update (attributes) ; } }
};
}

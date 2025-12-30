// Generated macro for impl_1254 (impl)
macro_rules! Depcrate_skipimpl_1254 {
() => {
// Module: crate::skip
// Provides: {"impl_1254"}
// Dependencies: {}
impl SkipContext { pub (crate) fn update_with_attrs (& mut self , attrs : & [ast :: Attribute]) { self . macros . extend (get_skip_names ("macros" , attrs)) ; self . attributes . extend (get_skip_names ("attributes" , attrs)) ; } pub (crate) fn update (& mut self , other : SkipContext) { let SkipContext { macros , attributes } = other ; self . macros . update (macros) ; self . attributes . update (attributes) ; } }
};
}

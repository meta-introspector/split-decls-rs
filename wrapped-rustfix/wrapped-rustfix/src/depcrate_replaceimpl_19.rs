// Generated macro for impl_19 (impl)
macro_rules! Depcrate_replaceimpl_19 {
() => {
// Module: crate::replace
// Provides: {"impl_19"}
// Dependencies: {}
impl std :: fmt :: Debug for Span { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let state = if self . is_insert () { "inserted" } else { "replaced" } ; let committed = if self . committed { "committed" } else { "uncommitted" } ; write ! (f , "({}, {}: {state}, {committed})" , self . range . start , self . range . end) } }
};
}

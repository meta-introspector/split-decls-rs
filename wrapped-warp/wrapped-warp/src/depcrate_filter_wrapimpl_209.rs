// Generated macro for impl_209 (impl)
macro_rules! Depcrate_filter_wrapimpl_209 {
() => {
// Module: crate::filter::wrap
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'a , T , F > WrapSealed < F > for & 'a T where T : WrapSealed < F > , F : Filter , { type Wrapped = T :: Wrapped ; fn wrap (& self , filter : F) -> Self :: Wrapped { (* self) . wrap (filter) } }
};
}

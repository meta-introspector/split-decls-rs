// Generated macro for impl_554 (impl)
macro_rules! Depcrate_filters_replyimpl_554 {
() => {
// Module: crate::filters::reply
// Provides: {"impl_554"}
// Dependencies: {}
impl < F , R > WrapSealed < F > for WithHeader where F : Filter < Extract = (R ,) > , R : Reply , { type Wrapped = Map < F , WithHeader_ > ; fn wrap (& self , filter : F) -> Self :: Wrapped { let with = WithHeader_ { with : self . clone () } ; filter . map (with) } }
};
}

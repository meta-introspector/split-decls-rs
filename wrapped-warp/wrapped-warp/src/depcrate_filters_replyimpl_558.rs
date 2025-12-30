// Generated macro for impl_558 (impl)
macro_rules! Depcrate_filters_replyimpl_558 {
() => {
// Module: crate::filters::reply
// Provides: {"impl_558"}
// Dependencies: {}
impl < F , R > WrapSealed < F > for WithDefaultHeader where F : Filter < Extract = (R ,) > , R : Reply , { type Wrapped = Map < F , WithDefaultHeader_ > ; fn wrap (& self , filter : F) -> Self :: Wrapped { let with = WithDefaultHeader_ { with : self . clone () } ; filter . map (with) } }
};
}

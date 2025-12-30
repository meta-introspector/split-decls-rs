// Generated macro for impl_556 (impl)
macro_rules! Depcrate_filters_replyimpl_556 {
() => {
// Module: crate::filters::reply
// Provides: {"impl_556"}
// Dependencies: {}
impl < F , R > WrapSealed < F > for WithHeaders where F : Filter < Extract = (R ,) > , R : Reply , { type Wrapped = Map < F , WithHeaders_ > ; fn wrap (& self , filter : F) -> Self :: Wrapped { let with = WithHeaders_ { with : self . clone () } ; filter . map (with) } }
};
}

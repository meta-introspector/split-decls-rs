// Generated macro for impl_341 (impl)
macro_rules! Depcrate_type_operatorsimpl_341 {
() => {
// Module: crate::type_operators
// Provides: {"impl_341"}
// Dependencies: {}
impl < A , B > IsGreater < B > for A where A : Cmp < B > + IsGreaterPrivate < B , Compare < A , B > > , { type Output = < A as IsGreaterPrivate < B , Compare < A , B > > > :: Output ; # [inline] fn is_greater (self , rhs : B) -> Self :: Output { let lhs_cmp_rhs = self . compare :: < Internal > (& rhs) ; self . is_greater_private (rhs , lhs_cmp_rhs) } }
};
}

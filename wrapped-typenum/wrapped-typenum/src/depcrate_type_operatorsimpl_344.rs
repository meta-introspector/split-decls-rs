// Generated macro for impl_344 (impl)
macro_rules! Depcrate_type_operatorsimpl_344 {
() => {
// Module: crate::type_operators
// Provides: {"impl_344"}
// Dependencies: {}
impl < A , B > IsLess < B > for A where A : Cmp < B > + IsLessPrivate < B , Compare < A , B > > , { type Output = < A as IsLessPrivate < B , Compare < A , B > > > :: Output ; # [inline] fn is_less (self , rhs : B) -> Self :: Output { let lhs_cmp_rhs = self . compare :: < Internal > (& rhs) ; self . is_less_private (rhs , lhs_cmp_rhs) } }
};
}

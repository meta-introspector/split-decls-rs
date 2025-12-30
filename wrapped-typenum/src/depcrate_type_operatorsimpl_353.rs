// Generated macro for impl_353 (impl)
macro_rules! Depcrate_type_operatorsimpl_353 {
() => {
// Module: crate::type_operators
// Provides: {"impl_353"}
// Dependencies: {}
impl < A , B > IsLessOrEqual < B > for A where A : Cmp < B > + IsLessOrEqualPrivate < B , Compare < A , B > > , { type Output = < A as IsLessOrEqualPrivate < B , Compare < A , B > > > :: Output ; # [inline] fn is_less_or_equal (self , rhs : B) -> Self :: Output { let lhs_cmp_rhs = self . compare :: < Internal > (& rhs) ; self . is_less_or_equal_private (rhs , lhs_cmp_rhs) } }
};
}

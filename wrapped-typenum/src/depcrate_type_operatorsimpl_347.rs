// Generated macro for impl_347 (impl)
macro_rules! Depcrate_type_operatorsimpl_347 {
() => {
// Module: crate::type_operators
// Provides: {"impl_347"}
// Dependencies: {}
impl < A , B > IsEqual < B > for A where A : Cmp < B > + IsEqualPrivate < B , Compare < A , B > > , { type Output = < A as IsEqualPrivate < B , Compare < A , B > > > :: Output ; # [inline] fn is_equal (self , rhs : B) -> Self :: Output { let lhs_cmp_rhs = self . compare :: < Internal > (& rhs) ; self . is_equal_private (rhs , lhs_cmp_rhs) } }
};
}

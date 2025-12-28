macro_rules! deps {
    () => {
        Compare!();
        Cmp!();
        Internal!();
        IsNotEqualPrivate!();
        IsNotEqual!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < A , B > IsNotEqual < B > for A where A : Cmp < B > + IsNotEqualPrivate < B , Compare < A , B > > , { type Output = < A as IsNotEqualPrivate < B , Compare < A , B > > > :: Output ; # [inline] fn is_not_equal (self , rhs : B) -> Self :: Output { let lhs_cmp_rhs = self . compare :: < Internal > (& rhs) ; self . is_not_equal_private (rhs , lhs_cmp_rhs) } }
    };
}

impl_329!();
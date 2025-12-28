macro_rules! deps {
    () => {
        Cmp!();
        IsGreaterOrEqual!();
        IsGreaterOrEqualPrivate!();
        Compare!();
        Internal!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl < A , B > IsGreaterOrEqual < B > for A where A : Cmp < B > + IsGreaterOrEqualPrivate < B , Compare < A , B > > , { type Output = < A as IsGreaterOrEqualPrivate < B , Compare < A , B > > > :: Output ; # [inline] fn is_greater_or_equal (self , rhs : B) -> Self :: Output { let lhs_cmp_rhs = self . compare :: < Internal > (& rhs) ; self . is_greater_or_equal_private (rhs , lhs_cmp_rhs) } }
    };
}

impl_331!();
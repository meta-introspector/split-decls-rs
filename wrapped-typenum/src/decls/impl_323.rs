macro_rules! deps {
    () => {
        IsEqualPrivate!();
        Internal!();
        Cmp!();
        IsEqual!();
        Compare!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl < A , B > IsEqual < B > for A where A : Cmp < B > + IsEqualPrivate < B , Compare < A , B > > , { type Output = < A as IsEqualPrivate < B , Compare < A , B > > > :: Output ; # [inline] fn is_equal (self , rhs : B) -> Self :: Output { let lhs_cmp_rhs = self . compare :: < Internal > (& rhs) ; self . is_equal_private (rhs , lhs_cmp_rhs) } }
    };
}

impl_323!()
macro_rules! deps {
    () => {
        IsLessPrivate!();
        IsLess!();
        Cmp!();
        Compare!();
        Internal!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl < A , B > IsLess < B > for A where A : Cmp < B > + IsLessPrivate < B , Compare < A , B > > , { type Output = < A as IsLessPrivate < B , Compare < A , B > > > :: Output ; # [inline] fn is_less (self , rhs : B) -> Self :: Output { let lhs_cmp_rhs = self . compare :: < Internal > (& rhs) ; self . is_less_private (rhs , lhs_cmp_rhs) } }
    };
}

impl_321!()
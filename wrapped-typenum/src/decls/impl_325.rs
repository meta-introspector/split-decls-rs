macro_rules! deps {
    () => {
        IsGreater!();
        Cmp!();
        Compare!();
        Internal!();
        IsGreaterPrivate!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl < A , B > IsGreater < B > for A where A : Cmp < B > + IsGreaterPrivate < B , Compare < A , B > > , { type Output = < A as IsGreaterPrivate < B , Compare < A , B > > > :: Output ; # [inline] fn is_greater (self , rhs : B) -> Self :: Output { let lhs_cmp_rhs = self . compare :: < Internal > (& rhs) ; self . is_greater_private (rhs , lhs_cmp_rhs) } }
    };
}

impl_325!()
macro_rules! deps {
    () => {
        PrivateIntegerAdd!();
        PInt!();
        NInt!();
        Cmp!();
        NonZero!();
        Unsigned!();
        Internal!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [doc = " `N(Ul) + P(Ur)`: We resolve this with our `PrivateAdd`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Add < PInt < Ur > > for NInt < Ul > where Ur : Cmp < Ul > + PrivateIntegerAdd < < Ur as Cmp < Ul > > :: Output , Ul > , { type Output = < Ur as PrivateIntegerAdd < < Ur as Cmp < Ul > > :: Output , Ul > > :: Output ; # [inline] fn add (self , rhs : PInt < Ur >) -> Self :: Output { let lhs = self . n ; let rhs = rhs . n ; let rhs_cmp_lhs = rhs . compare :: < Internal > (& lhs) ; rhs . private_integer_add (rhs_cmp_lhs , lhs) } }
    };
}

impl_65!()
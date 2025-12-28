macro_rules! deps {
    () => {
        Cmp!();
        PrivateIntegerAdd!();
        Internal!();
        Unsigned!();
        NonZero!();
        NInt!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        # [doc = " `N(Ul) - N(Ur)`: We resolve this with our `PrivateAdd`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Sub < NInt < Ur > > for NInt < Ul > where Ur : Cmp < Ul > + PrivateIntegerAdd < < Ur as Cmp < Ul > > :: Output , Ul > , { type Output = < Ur as PrivateIntegerAdd < < Ur as Cmp < Ul > > :: Output , Ul > > :: Output ; # [inline] fn sub (self , rhs : NInt < Ur >) -> Self :: Output { let lhs = self . n ; let rhs = rhs . n ; let rhs_cmp_lhs = rhs . compare :: < Internal > (& lhs) ; rhs . private_integer_add (rhs_cmp_lhs , lhs) } }
    };
}

impl_77!();
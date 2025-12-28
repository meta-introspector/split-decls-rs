macro_rules! deps {
    () => {
        Cmp!();
        NonZero!();
        Internal!();
        PInt!();
        PrivateIntegerAdd!();
        Unsigned!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        # [doc = " `P(Ul) - P(Ur)`: We resolve this with our `PrivateAdd`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Sub < PInt < Ur > > for PInt < Ul > where Ul : Cmp < Ur > + PrivateIntegerAdd < < Ul as Cmp < Ur > > :: Output , Ur > , { type Output = < Ul as PrivateIntegerAdd < < Ul as Cmp < Ur > > :: Output , Ur > > :: Output ; # [inline] fn sub (self , rhs : PInt < Ur >) -> Self :: Output { let lhs = self . n ; let rhs = rhs . n ; let lhs_cmp_rhs = lhs . compare :: < Internal > (& rhs) ; lhs . private_integer_add (lhs_cmp_rhs , rhs) } }
    };
}

impl_76!();
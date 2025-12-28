macro_rules! deps {
    () => {
        Unsigned!();
        NonZero!();
        Cmp!();
        PrivateIntegerAdd!();
        Internal!();
        PInt!();
        NInt!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        # [doc = " `P(Ul) + N(Ur)`: We resolve this with our `PrivateAdd`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Add < NInt < Ur > > for PInt < Ul > where Ul : Cmp < Ur > + PrivateIntegerAdd < < Ul as Cmp < Ur > > :: Output , Ur > , { type Output = < Ul as PrivateIntegerAdd < < Ul as Cmp < Ur > > :: Output , Ur > > :: Output ; # [inline] fn add (self , rhs : NInt < Ur >) -> Self :: Output { let lhs = self . n ; let rhs = rhs . n ; let lhs_cmp_rhs = lhs . compare :: < Internal > (& rhs) ; lhs . private_integer_add (lhs_cmp_rhs , rhs) } }
    };
}

impl_64!()
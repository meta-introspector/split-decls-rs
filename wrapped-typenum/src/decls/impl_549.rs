macro_rules! deps {
    () => {
        NonZero!();
        Prod!();
        PInt!();
        Unsigned!();
        TArr!();
    };
}

macro_rules! impl_549 {
    () => {
        deps!();
        impl < V , A , U > Mul < TArr < V , A > > for PInt < U > where U : Unsigned + NonZero , PInt < U > : Mul < A > + Mul < V > , { type Output = TArr < Prod < PInt < U > , V > , Prod < PInt < U > , A > > ; # [inline] fn mul (self , rhs : TArr < V , A >) -> Self :: Output { TArr { first : self * rhs . first , rest : self * rhs . rest , } } }
    };
}

impl_549!()
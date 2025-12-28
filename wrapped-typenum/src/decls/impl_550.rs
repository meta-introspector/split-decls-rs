macro_rules! deps {
    () => {
        Prod!();
        NInt!();
        TArr!();
        Unsigned!();
        NonZero!();
    };
}

macro_rules! impl_550 {
    () => {
        deps!();
        impl < V , A , U > Mul < TArr < V , A > > for NInt < U > where U : Unsigned + NonZero , NInt < U > : Mul < A > + Mul < V > , { type Output = TArr < Prod < NInt < U > , V > , Prod < NInt < U > , A > > ; # [inline] fn mul (self , rhs : TArr < V , A >) -> Self :: Output { TArr { first : self * rhs . first , rest : self * rhs . rest , } } }
    };
}

impl_550!()
macro_rules! deps {
    () => {
        Prod!();
        TArr!();
    };
}

macro_rules! impl_544 {
    () => {
        deps!();
        impl < V , A , Rhs > Mul < Rhs > for TArr < V , A > where V : Mul < Rhs > , A : Mul < Rhs > , Rhs : Copy , { type Output = TArr < Prod < V , Rhs > , Prod < A , Rhs > > ; # [inline] fn mul (self , rhs : Rhs) -> Self :: Output { TArr { first : self . first * rhs , rest : self . rest * rhs , } } }
    };
}

impl_544!();
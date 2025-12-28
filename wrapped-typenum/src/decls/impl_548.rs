macro_rules! deps {
    () => {
        Prod!();
        Z0!();
        TArr!();
    };
}

macro_rules! impl_548 {
    () => {
        deps!();
        impl < V , A > Mul < TArr < V , A > > for Z0 where Z0 : Mul < A > , { type Output = TArr < Z0 , Prod < Z0 , A > > ; # [inline] fn mul (self , rhs : TArr < V , A >) -> Self :: Output { TArr { first : Z0 , rest : self * rhs . rest , } } }
    };
}

impl_548!()
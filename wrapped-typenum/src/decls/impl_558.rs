macro_rules! deps {
    () => {
        Negate!();
        TArr!();
    };
}

macro_rules! impl_558 {
    () => {
        deps!();
        impl < V , A > Neg for TArr < V , A > where V : Neg , A : Neg , { type Output = TArr < Negate < V > , Negate < A > > ; # [inline] fn neg (self) -> Self :: Output { TArr { first : - self . first , rest : - self . rest , } } }
    };
}

impl_558!();
macro_rules! deps {
    () => {
        Quot!();
        TArr!();
    };
}

macro_rules! impl_552 {
    () => {
        deps!();
        impl < V , A , Rhs > Div < Rhs > for TArr < V , A > where V : Div < Rhs > , A : Div < Rhs > , Rhs : Copy , { type Output = TArr < Quot < V , Rhs > , Quot < A , Rhs > > ; # [inline] fn div (self , rhs : Rhs) -> Self :: Output { TArr { first : self . first / rhs , rest : self . rest / rhs , } } }
    };
}

impl_552!()
macro_rules! deps {
    () => {
        TArr!();
        Mod!();
    };
}

macro_rules! impl_556 {
    () => {
        deps!();
        impl < V , A , Rhs > Rem < Rhs > for TArr < V , A > where V : Rem < Rhs > , A : Rem < Rhs > , Rhs : Copy , { type Output = TArr < Mod < V , Rhs > , Mod < A , Rhs > > ; # [inline] fn rem (self , rhs : Rhs) -> Self :: Output { TArr { first : self . first % rhs , rest : self . rest % rhs , } } }
    };
}

impl_556!()
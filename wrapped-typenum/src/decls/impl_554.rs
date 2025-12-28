macro_rules! deps {
    () => {
        PartialDiv!();
        PartialQuot!();
        TArr!();
    };
}

macro_rules! impl_554 {
    () => {
        deps!();
        impl < V , A , Rhs > PartialDiv < Rhs > for TArr < V , A > where V : PartialDiv < Rhs > , A : PartialDiv < Rhs > , Rhs : Copy , { type Output = TArr < PartialQuot < V , Rhs > , PartialQuot < A , Rhs > > ; # [inline] fn partial_div (self , rhs : Rhs) -> Self :: Output { TArr { first : self . first . partial_div (rhs) , rest : self . rest . partial_div (rhs) , } } }
    };
}

impl_554!()
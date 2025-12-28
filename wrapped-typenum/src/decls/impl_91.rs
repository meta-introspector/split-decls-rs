macro_rules! deps {
    () => {
        PartialDiv!();
        Quot!();
        Z0!();
        Integer!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < M , N > PartialDiv < N > for M where M : Integer + Div < N > + Rem < N , Output = Z0 > , { type Output = Quot < M , N > ; # [inline] fn partial_div (self , rhs : N) -> Self :: Output { self / rhs } }
    };
}

impl_91!();
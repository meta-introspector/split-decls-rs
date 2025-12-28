macro_rules! deps {
    () => {
        Diff!();
        TArr!();
    };
}

macro_rules! impl_542 {
    () => {
        deps!();
        impl < Vl , Al , Vr , Ar > Sub < TArr < Vr , Ar > > for TArr < Vl , Al > where Vl : Sub < Vr > , Al : Sub < Ar > , { type Output = TArr < Diff < Vl , Vr > , Diff < Al , Ar > > ; # [inline] fn sub (self , rhs : TArr < Vr , Ar >) -> Self :: Output { TArr { first : self . first - rhs . first , rest : self . rest - rhs . rest , } } }
    };
}

impl_542!();
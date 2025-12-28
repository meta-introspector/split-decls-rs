macro_rules! deps {
    () => {
        TArr!();
        Sum!();
    };
}

macro_rules! impl_540 {
    () => {
        deps!();
        impl < Al , Vl , Ar , Vr > Add < TArr < Vr , Ar > > for TArr < Vl , Al > where Al : Add < Ar > , Vl : Add < Vr > , { type Output = TArr < Sum < Vl , Vr > , Sum < Al , Ar > > ; # [inline] fn add (self , rhs : TArr < Vr , Ar >) -> Self :: Output { TArr { first : self . first + rhs . first , rest : self . rest + rhs . rest , } } }
    };
}

impl_540!();
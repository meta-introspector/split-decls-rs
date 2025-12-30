// Generated macro for impl_565 (impl)
macro_rules! Depcrate_arrayimpl_565 {
() => {
// Module: crate::array
// Provides: {"impl_565"}
// Dependencies: {}
impl < Al , Vl , Ar , Vr > Add < TArr < Vr , Ar > > for TArr < Vl , Al > where Al : Add < Ar > , Vl : Add < Vr > , { type Output = TArr < Sum < Vl , Vr > , Sum < Al , Ar > > ; # [inline] fn add (self , rhs : TArr < Vr , Ar >) -> Self :: Output { TArr { first : self . first + rhs . first , rest : self . rest + rhs . rest , } } }
};
}

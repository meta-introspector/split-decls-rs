// Generated macro for impl_586 (impl)
macro_rules! Depcrate_arrayimpl_586 {
() => {
// Module: crate::array
// Provides: {"impl_586"}
// Dependencies: {}
impl < Al , Vl , Ar , Vr > Add < TArr < Vr , Ar > > for TArr < Vl , Al > where Al : Add < Ar > , Vl : Add < Vr > , { type Output = TArr < Sum < Vl , Vr > , Sum < Al , Ar > > ; # [inline] fn add (self , rhs : TArr < Vr , Ar >) -> Self :: Output { TArr { first : self . first + rhs . first , rest : self . rest + rhs . rest , } } }
};
}

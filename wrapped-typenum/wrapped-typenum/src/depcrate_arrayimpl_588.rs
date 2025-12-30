// Generated macro for impl_588 (impl)
macro_rules! Depcrate_arrayimpl_588 {
() => {
// Module: crate::array
// Provides: {"impl_588"}
// Dependencies: {}
impl < Vl , Al , Vr , Ar > Sub < TArr < Vr , Ar > > for TArr < Vl , Al > where Vl : Sub < Vr > , Al : Sub < Ar > , { type Output = TArr < Diff < Vl , Vr > , Diff < Al , Ar > > ; # [inline] fn sub (self , rhs : TArr < Vr , Ar >) -> Self :: Output { TArr { first : self . first - rhs . first , rest : self . rest - rhs . rest , } } }
};
}

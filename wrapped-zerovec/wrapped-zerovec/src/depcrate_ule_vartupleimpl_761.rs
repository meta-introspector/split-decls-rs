// Generated macro for impl_761 (impl)
macro_rules! Depcrate_ule_vartupleimpl_761 {
() => {
// Module: crate::ule::vartuple
// Provides: {"impl_761"}
// Dependencies: {}
impl < 'a , A , B , V > ZeroFrom < 'a , VarTupleULE < A , V > > for VarTuple < A , B > where A : AsULE + 'static , V : VarULE + ? Sized , B : ZeroFrom < 'a , V > , { fn zero_from (other : & 'a VarTupleULE < A , V >) -> Self { VarTuple { sized : AsULE :: from_unaligned (other . sized) , variable : B :: zero_from (& other . variable) , } } }
};
}

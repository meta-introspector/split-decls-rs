// Generated macro for impl_109 (impl)
macro_rules! Depcrate_intimpl_109 {
() => {
// Module: crate::int
// Provides: {"impl_109"}
// Dependencies: {}
# [doc = " -X <==> -Y"] impl < Nl : Unsigned + NonZero , Nr : Cmp < Nl > + Unsigned + NonZero > Cmp < NInt < Nr > > for NInt < Nl > { type Output = < Nr as Cmp < Nl > > :: Output ; # [inline] fn compare < IM : InternalMarker > (& self , rhs : & NInt < Nr >) -> Self :: Output { rhs . n . compare :: < Internal > (& self . n) } }
};
}

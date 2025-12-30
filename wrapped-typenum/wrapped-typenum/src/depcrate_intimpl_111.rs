// Generated macro for impl_111 (impl)
macro_rules! Depcrate_intimpl_111 {
() => {
// Module: crate::int
// Provides: {"impl_111"}
// Dependencies: {}
# [doc = " X <==> Y"] impl < Pl : Cmp < Pr > + Unsigned + NonZero , Pr : Unsigned + NonZero > Cmp < PInt < Pr > > for PInt < Pl > { type Output = < Pl as Cmp < Pr > > :: Output ; # [inline] fn compare < IM : InternalMarker > (& self , rhs : & PInt < Pr >) -> Self :: Output { self . n . compare :: < Internal > (& rhs . n) } }
};
}

// Generated macro for impl_149 (impl)
macro_rules! Depcrate_intimpl_149 {
() => {
// Module: crate::int
// Provides: {"impl_149"}
// Dependencies: {}
impl < Ul , Ur > Min < NInt < Ur > > for NInt < Ul > where Ul : Unsigned + NonZero + Max < Ur > , Ur : Unsigned + NonZero , Maximum < Ul , Ur > : Unsigned + NonZero , { type Output = NInt < Maximum < Ul , Ur > > ; # [inline] fn min (self , rhs : NInt < Ur >) -> Self :: Output { NInt { n : self . n . max (rhs . n) , } } }
};
}

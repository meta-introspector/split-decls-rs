// Generated macro for impl_152 (impl)
macro_rules! Depcrate_intimpl_152 {
() => {
// Module: crate::int
// Provides: {"impl_152"}
// Dependencies: {}
impl < Ul , Ur > Max < PInt < Ur > > for PInt < Ul > where Ul : Unsigned + NonZero + Max < Ur > , Ur : Unsigned + NonZero , Maximum < Ul , Ur > : Unsigned + NonZero , { type Output = PInt < Maximum < Ul , Ur > > ; # [inline] fn max (self , rhs : PInt < Ur >) -> Self :: Output { PInt { n : self . n . max (rhs . n) , } } }
};
}

// Generated macro for impl_143 (impl)
macro_rules! Depcrate_intimpl_143 {
() => {
// Module: crate::int
// Provides: {"impl_143"}
// Dependencies: {}
impl < Ul , Ur > Min < PInt < Ur > > for PInt < Ul > where Ul : Unsigned + NonZero + Min < Ur > , Ur : Unsigned + NonZero , Minimum < Ul , Ur > : Unsigned + NonZero , { type Output = PInt < Minimum < Ul , Ur > > ; # [inline] fn min (self , rhs : PInt < Ur >) -> Self :: Output { PInt { n : self . n . min (rhs . n) , } } }
};
}

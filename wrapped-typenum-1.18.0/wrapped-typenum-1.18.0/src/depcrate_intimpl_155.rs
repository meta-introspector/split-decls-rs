// Generated macro for impl_155 (impl)
macro_rules! Depcrate_intimpl_155 {
() => {
// Module: crate::int
// Provides: {"impl_155"}
// Dependencies: {}
impl < Ul , Ur > Max < NInt < Ur > > for NInt < Ul > where Ul : Unsigned + NonZero + Min < Ur > , Ur : Unsigned + NonZero , Minimum < Ul , Ur > : Unsigned + NonZero , { type Output = NInt < Minimum < Ul , Ur > > ; # [inline] fn max (self , rhs : NInt < Ur >) -> Self :: Output { NInt { n : self . n . min (rhs . n) , } } }
};
}

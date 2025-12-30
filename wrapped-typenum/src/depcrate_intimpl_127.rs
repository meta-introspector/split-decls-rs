// Generated macro for impl_127 (impl)
macro_rules! Depcrate_intimpl_127 {
() => {
// Module: crate::int
// Provides: {"impl_127"}
// Dependencies: {}
# [doc = " P(Ul)^P(Ur) = P(Ul^Ur)"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Pow < PInt < Ur > > for PInt < Ul > where Ul : Pow < Ur > , < Ul as Pow < Ur > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Pow < Ur > > :: Output > ; # [inline] fn powi (self , _ : PInt < Ur >) -> Self :: Output { PInt :: new () } }
};
}

// Generated macro for impl_92 (impl)
macro_rules! Depcrate_intimpl_92 {
() => {
// Module: crate::int
// Provides: {"impl_92"}
// Dependencies: {}
# [doc = " P(Ul) * P(Ur) = P(Ul * Ur)"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Mul < PInt < Ur > > for PInt < Ul > where Ul : Mul < Ur > , < Ul as Mul < Ur > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Mul < Ur > > :: Output > ; # [inline] fn mul (self , _ : PInt < Ur >) -> Self :: Output { PInt :: new () } }
};
}

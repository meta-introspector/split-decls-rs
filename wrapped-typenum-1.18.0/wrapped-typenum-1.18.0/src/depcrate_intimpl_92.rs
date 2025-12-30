// Generated macro for impl_92 (impl)
macro_rules! Depcrate_intimpl_92 {
() => {
// Module: crate::int
// Provides: {"impl_92"}
// Dependencies: {}
# [doc = " N(Ul) * P(Ur) = N(Ul * Ur)"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Mul < PInt < Ur > > for NInt < Ul > where Ul : Mul < Ur > , < Ul as Mul < Ur > > :: Output : Unsigned + NonZero , { type Output = NInt < < Ul as Mul < Ur > > :: Output > ; # [inline] fn mul (self , _ : PInt < Ur >) -> Self :: Output { NInt :: new () } }
};
}

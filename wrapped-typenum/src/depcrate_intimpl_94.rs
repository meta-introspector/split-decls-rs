// Generated macro for impl_94 (impl)
macro_rules! Depcrate_intimpl_94 {
() => {
// Module: crate::int
// Provides: {"impl_94"}
// Dependencies: {}
# [doc = " P(Ul) * N(Ur) = N(Ul * Ur)"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Mul < NInt < Ur > > for PInt < Ul > where Ul : Mul < Ur > , < Ul as Mul < Ur > > :: Output : Unsigned + NonZero , { type Output = NInt < < Ul as Mul < Ur > > :: Output > ; # [inline] fn mul (self , _ : NInt < Ur >) -> Self :: Output { NInt :: new () } }
};
}

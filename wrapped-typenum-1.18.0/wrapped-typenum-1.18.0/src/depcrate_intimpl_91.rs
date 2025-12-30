// Generated macro for impl_91 (impl)
macro_rules! Depcrate_intimpl_91 {
() => {
// Module: crate::int
// Provides: {"impl_91"}
// Dependencies: {}
# [doc = " P(Ul) * N(Ur) = N(Ul * Ur)"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Mul < NInt < Ur > > for PInt < Ul > where Ul : Mul < Ur > , < Ul as Mul < Ur > > :: Output : Unsigned + NonZero , { type Output = NInt < < Ul as Mul < Ur > > :: Output > ; # [inline] fn mul (self , _ : NInt < Ur >) -> Self :: Output { NInt :: new () } }
};
}

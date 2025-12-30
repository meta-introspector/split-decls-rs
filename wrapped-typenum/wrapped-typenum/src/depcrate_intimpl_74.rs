// Generated macro for impl_74 (impl)
macro_rules! Depcrate_intimpl_74 {
() => {
// Module: crate::int
// Provides: {"impl_74"}
// Dependencies: {}
# [doc = " `N(Ul) + N(Ur) = N(Ul + Ur)`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Add < NInt < Ur > > for NInt < Ul > where Ul : Add < Ur > , < Ul as Add < Ur > > :: Output : Unsigned + NonZero , { type Output = NInt < < Ul as Add < Ur > > :: Output > ; # [inline] fn add (self , _ : NInt < Ur >) -> Self :: Output { NInt :: new () } }
};
}

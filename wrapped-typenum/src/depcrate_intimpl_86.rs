// Generated macro for impl_86 (impl)
macro_rules! Depcrate_intimpl_86 {
() => {
// Module: crate::int
// Provides: {"impl_86"}
// Dependencies: {}
# [doc = " `N(Ul) - P(Ur) = N(Ul + Ur)`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Sub < PInt < Ur > > for NInt < Ul > where Ul : Add < Ur > , < Ul as Add < Ur > > :: Output : Unsigned + NonZero , { type Output = NInt < < Ul as Add < Ur > > :: Output > ; # [inline] fn sub (self , _ : PInt < Ur >) -> Self :: Output { NInt :: new () } }
};
}

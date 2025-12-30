// Generated macro for impl_82 (impl)
macro_rules! Depcrate_intimpl_82 {
() => {
// Module: crate::int
// Provides: {"impl_82"}
// Dependencies: {}
# [doc = " `P(Ul) - N(Ur) = P(Ul + Ur)`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Sub < NInt < Ur > > for PInt < Ul > where Ul : Add < Ur > , < Ul as Add < Ur > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Add < Ur > > :: Output > ; # [inline] fn sub (self , _ : NInt < Ur >) -> Self :: Output { PInt :: new () } }
};
}

// Generated macro for impl_85 (impl)
macro_rules! Depcrate_intimpl_85 {
() => {
// Module: crate::int
// Provides: {"impl_85"}
// Dependencies: {}
# [doc = " `P(Ul) - N(Ur) = P(Ul + Ur)`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Sub < NInt < Ur > > for PInt < Ul > where Ul : Add < Ur > , < Ul as Add < Ur > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Add < Ur > > :: Output > ; # [inline] fn sub (self , _ : NInt < Ur >) -> Self :: Output { PInt :: new () } }
};
}

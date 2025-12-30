// Generated macro for impl_70 (impl)
macro_rules! Depcrate_intimpl_70 {
() => {
// Module: crate::int
// Provides: {"impl_70"}
// Dependencies: {}
# [doc = " `P(Ul) + P(Ur) = P(Ul + Ur)`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Add < PInt < Ur > > for PInt < Ul > where Ul : Add < Ur > , < Ul as Add < Ur > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Add < Ur > > :: Output > ; # [inline] fn add (self , _ : PInt < Ur >) -> Self :: Output { PInt :: new () } }
};
}

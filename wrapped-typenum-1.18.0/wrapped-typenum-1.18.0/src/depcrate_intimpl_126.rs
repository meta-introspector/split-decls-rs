// Generated macro for impl_126 (impl)
macro_rules! Depcrate_intimpl_126 {
() => {
// Module: crate::int
// Provides: {"impl_126"}
// Dependencies: {}
# [doc = " N(Ul)^P(Ur) = N(Ul^Ur) if Ur is odd"] impl < Ul : Unsigned + NonZero , Ur : Unsigned > Pow < PInt < UInt < Ur , B1 > > > for NInt < Ul > where Ul : Pow < UInt < Ur , B1 > > , < Ul as Pow < UInt < Ur , B1 > > > :: Output : Unsigned + NonZero , { type Output = NInt < < Ul as Pow < UInt < Ur , B1 > > > :: Output > ; # [inline] fn powi (self , _ : PInt < UInt < Ur , B1 > >) -> Self :: Output { NInt :: new () } }
};
}

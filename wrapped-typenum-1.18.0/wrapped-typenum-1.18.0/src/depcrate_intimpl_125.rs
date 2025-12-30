// Generated macro for impl_125 (impl)
macro_rules! Depcrate_intimpl_125 {
() => {
// Module: crate::int
// Provides: {"impl_125"}
// Dependencies: {}
# [doc = " N(Ul)^P(Ur) = P(Ul^Ur) if Ur is even"] impl < Ul : Unsigned + NonZero , Ur : Unsigned > Pow < PInt < UInt < Ur , B0 > > > for NInt < Ul > where Ul : Pow < UInt < Ur , B0 > > , < Ul as Pow < UInt < Ur , B0 > > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Pow < UInt < Ur , B0 > > > :: Output > ; # [inline] fn powi (self , _ : PInt < UInt < Ur , B0 > >) -> Self :: Output { PInt :: new () } }
};
}

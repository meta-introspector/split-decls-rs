// Generated macro for impl_474 (impl)
macro_rules! Depcrate_uintimpl_474 {
() => {
// Module: crate::uint
// Provides: {"impl_474"}
// Dependencies: {}
impl < Un , Bn > GetBit < U0 > for UInt < Un , Bn > where Bn : Copy , { type Output = Bn ; # [inline] fn get_bit < IM : InternalMarker > (& self , _ : & U0) -> Self :: Output { self . lsb } }
};
}

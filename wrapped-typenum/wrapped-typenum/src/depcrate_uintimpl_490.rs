// Generated macro for impl_490 (impl)
macro_rules! Depcrate_uintimpl_490 {
() => {
// Module: crate::uint
// Provides: {"impl_490"}
// Dependencies: {}
impl < Un , Bn , Ui , Bi > GetBit < UInt < Ui , Bi > > for UInt < Un , Bn > where UInt < Ui , Bi > : Copy + Sub < B1 > , Un : GetBit < Sub1 < UInt < Ui , Bi > > > , { type Output = GetBitOut < Un , Sub1 < UInt < Ui , Bi > > > ; # [inline] fn get_bit < IM : InternalMarker > (& self , i : & UInt < Ui , Bi >) -> Self :: Output { self . msb . get_bit :: < Internal > (& (* i - B1)) } }
};
}

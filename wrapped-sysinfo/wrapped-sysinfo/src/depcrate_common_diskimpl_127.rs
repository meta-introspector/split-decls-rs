// Generated macro for impl_127 (impl)
macro_rules! Depcrate_common_diskimpl_127 {
() => {
// Module: crate::common::disk
// Provides: {"impl_127"}
// Dependencies: {}
impl fmt :: Display for DiskKind { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match * self { DiskKind :: HDD => "HDD" , DiskKind :: SSD => "SSD" , _ => "Unknown" , }) } }
};
}

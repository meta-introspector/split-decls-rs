// Generated macro for impl_351 (impl)
macro_rules! Depcrate_b_vtbl_4impl_351 {
() => {
// Module: crate::b_vtbl_4
// Provides: {"impl_351"}
// Dependencies: {}
impl GUID { pub const fn from_u128 (uuid : u128) -> Self { Self { data1 : (uuid >> 96) as u32 , data2 : (uuid >> 80 & 0xffff) as u16 , data3 : (uuid >> 64 & 0xffff) as u16 , data4 : (uuid as u64) . to_be_bytes () , } } }
};
}

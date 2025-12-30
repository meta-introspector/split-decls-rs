// Generated macro for impl_490 (impl)
macro_rules! Depcrate_legacy_shrinkimpl_490 {
() => {
// Module: crate::legacy::shrink
// Provides: {"impl_490"}
// Dependencies: {}
impl Codetab { pub fn create_new () -> [Self ; MAX_CODE + 1] { let mut codetab = [Codetab :: default () ; MAX_CODE + 1] ; for (i , code) in codetab . iter_mut () . enumerate () . take (u8 :: MAX as usize + 1) { * code = Codetab { prefix_code : Some (i as u16) , ext_byte : i as u8 , len : 1 , last_dst_pos : 0 , } ; } codetab } }
};
}

// Generated macro for from_any (function)
macro_rules! Depcrate_internal_cast_primitivefrom_any {
() => {
// Module: crate::internal::cast::primitive
// Provides: {"from_any"}
// Dependencies: {}
pub (in crate :: internal) fn from_any < 'v , T : ? Sized + 'static > (value : & 'v T) -> Option < ValueBag < 'v > > { let type_ids = | v : VoidRef < 'v > | { if TypeId :: of :: < T > () == TypeId :: of :: < str > () { let v = unsafe { * (v . 0 as * const & 'v str) } ; return Some (ValueBag :: from (v)) ; } check_type_ids ! (&'v v => usize , u8 , u16 , u32 , u64 , u128 , isize , i8 , i16 , i32 , i64 , i128 , f32 , f64 , char , bool , &'static str , # [cfg (feature = "alloc")] String ,) ; None } ; (type_ids) (VoidRef (& (value) as * const & 'v T as * const & 'v Void)) }
};
}

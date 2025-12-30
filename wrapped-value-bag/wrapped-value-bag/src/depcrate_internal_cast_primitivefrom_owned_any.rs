// Generated macro for from_owned_any (function)
macro_rules! Depcrate_internal_cast_primitivefrom_owned_any {
() => {
// Module: crate::internal::cast::primitive
// Provides: {"from_owned_any"}
// Dependencies: {}
# [cfg (feature = "owned")] pub (in crate :: internal) fn from_owned_any < 'a , T : ? Sized + 'static > (value : & 'a T ,) -> Option < ValueBag < 'static > > { let type_ids = | v : VoidRef < 'a > | { check_type_ids ! (&'a v => usize , u8 , u16 , u32 , u64 , # [cfg (feature = "inline-i128")] u128 , isize , i8 , i16 , i32 , i64 , # [cfg (feature = "inline-i128")] i128 , f32 , f64 , char , bool ,) ; None } ; (type_ids) (VoidRef (& (value) as * const & 'a T as * const & 'a Void)) }
};
}

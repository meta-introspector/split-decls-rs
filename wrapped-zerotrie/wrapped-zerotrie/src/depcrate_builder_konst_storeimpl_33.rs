// Generated macro for impl_33 (impl)
macro_rules! Depcrate_builder_konst_storeimpl_33 {
() => {
// Module: crate::builder::konst::store
// Provides: {"impl_33"}
// Dependencies: {}
impl < const N : usize , T : Copy > ConstArrayBuilder < N , T > { # [doc = " Takes a fully initialized builder as an array. Panics if the builder is not"] # [doc = " fully initialized."] pub const fn const_build_or_panic (self) -> [T ; N] { if self . start != 0 || self . limit != N { let actual_len = self . limit - self . start ; const PREFIX : & [u8 ; 31] = b"Buffer too large. Size needed: " ; let len_bytes : [u8 ; PREFIX . len () + crate :: helpers :: MAX_USIZE_LEN_AS_DIGITS] = crate :: helpers :: const_fmt_int (* PREFIX , actual_len) ; let Ok (len_str) = core :: str :: from_utf8 (& len_bytes) else { unreachable ! () } ; panic ! ("{}" , len_str) ; } self . full_array } # [doc = " Prepends an element to the front of the builder, panicking if there is no room."] # [allow (clippy :: indexing_slicing)] pub const fn const_push_front_or_panic (mut self , value : T) -> Self { if self . start == 0 { panic ! ("Buffer too small") ; } self . start -= 1 ; self . full_array [self . start] = value ; self } # [doc = " Prepends multiple elements to the front of the builder, panicking if there is no room."] # [allow (clippy :: indexing_slicing)] pub const fn const_extend_front_or_panic (mut self , other : ConstSlice < T >) -> Self { if self . start < other . len () { panic ! ("Buffer too small") ; } self . start -= other . len () ; let mut i = self . start ; const_for_each ! (other , byte , { self . full_array [i] = * byte ; i += 1 ; }) ; self } }
};
}

// Generated macro for impl_77 (impl)
macro_rules! Depcrate_builder_nonconst_storeimpl_77 {
() => {
// Module: crate::builder::nonconst::store
// Provides: {"impl_77"}
// Dependencies: {}
impl TrieBuilderStore for VecDeque < u8 > { fn atbs_new_empty () -> Self { VecDeque :: new () } fn atbs_len (& self) -> usize { self . len () } fn atbs_push_front (& mut self , byte : u8) { self . push_front (byte) ; } fn atbs_extend_front (& mut self , other : & [u8]) { self . reserve (other . len ()) ; for b in other . iter () . rev () { self . push_front (* b) ; } } fn atbs_to_bytes (& self) -> Vec < u8 > { let mut v = Vec :: with_capacity (self . len ()) ; let (a , b) = self . as_slices () ; v . extend (a) ; v . extend (b) ; v } fn atbs_bitor_assign (& mut self , index : usize , bits : u8) { self [index] |= bits ; } # [doc = " # Panics"] # [doc = " Panics if the specified ranges are invalid."] # [allow (clippy :: panic)] fn atbs_swap_ranges (& mut self , mut start : usize , mut mid : usize , mut limit : usize) { if start > mid || mid > limit { panic ! ("Invalid args to atbs_swap_ranges(): start > mid || mid > limit") ; } if limit > self . len () { panic ! ("Invalid args to atbs_swap_ranges(): limit out of range: {limit} > {}" , self . len ()) ; } loop { if start == mid || mid == limit { return ; } let len0 = mid - start ; let len1 = limit - mid ; let mut i = start ; let mut j = limit - core :: cmp :: min (len0 , len1) ; while j < limit { self . swap (i , j) ; i += 1 ; j += 1 ; } if len0 < len1 { mid = start + len0 ; limit -= len0 ; } else { start += len1 ; mid = limit - len1 ; } } } fn atbs_pop_front (& mut self) -> Option < u8 > { self . pop_front () } }
};
}

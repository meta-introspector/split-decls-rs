// Generated macro for impl_611 (impl)
macro_rules! Depcrate_sso_setimpl_611 {
() => {
// Module: crate::sso::set
// Provides: {"impl_611"}
// Dependencies: {}
impl < T > SsoHashSet < T > { # [doc = " Creates an empty `SsoHashSet`."] # [inline] pub fn new () -> Self { Self { map : SsoHashMap :: new () } } # [doc = " Creates an empty `SsoHashSet` with the specified capacity."] # [inline] pub fn with_capacity (cap : usize) -> Self { Self { map : SsoHashMap :: with_capacity (cap) } } # [doc = " Clears the set, removing all values."] # [inline] pub fn clear (& mut self) { self . map . clear () } # [doc = " Returns the number of elements the set can hold without reallocating."] # [inline] pub fn capacity (& self) -> usize { self . map . capacity () } # [doc = " Returns the number of elements in the set."] # [inline] pub fn len (& self) -> usize { self . map . len () } # [doc = " Returns `true` if the set contains no elements."] # [inline] pub fn is_empty (& self) -> bool { self . map . is_empty () } # [doc = " An iterator visiting all elements in arbitrary order."] # [doc = " The iterator element type is `&'a T`."] # [inline] pub fn iter (& self) -> impl Iterator < Item = & T > { self . into_iter () } # [doc = " Clears the set, returning all elements in an iterator."] # [inline] pub fn drain (& mut self) -> impl Iterator < Item = T > { self . map . drain () . map (entry_to_key) } }
};
}

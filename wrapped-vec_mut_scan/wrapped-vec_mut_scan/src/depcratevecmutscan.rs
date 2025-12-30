// Generated macro for VecMutScan (struct)
macro_rules! DepcrateVecMutScan {
() => {
// Module: crate
// Provides: {"VecMutScan"}
// Dependencies: {}
# [doc = " Forward scan over a vector with mutation and item removal."] # [doc = ""] # [doc = " Provides an iterator like interface over a vector which allows mutation and removal of items."] # [doc = ""] # [doc = " If you need to also add new elements, see [`VecGrowScan`]."] # [doc = ""] # [doc = " Items are kept in order and every item is moved at most once, even when items are removed."] # [doc = " Dropping the `VecMutScan` mid-iteration keeps remaining items in the vector."] # [doc = ""] # [doc = " This does not implement the iterator trait, as the returned items borrow from this (i.e. this is"] # [doc = " a streaming iterator)."] # [doc = ""] # [doc = " The [`next`](VecMutScan::next) method returns [`VecMutScanItem`] values, which auto dereference"] # [doc = " to the vector's item type but also provide a [`remove`](VecMutScanItem::remove) and"] # [doc = " [`replace`](VecMutScanItem::replace) method."] pub struct VecMutScan < 'a , T : 'a > { vec : & 'a mut Vec < T > , base : * mut T , write : usize , read : usize , end : usize , }
};
}

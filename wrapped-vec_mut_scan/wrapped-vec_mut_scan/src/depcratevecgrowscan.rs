// Generated macro for VecGrowScan (struct)
macro_rules! DepcrateVecGrowScan {
() => {
// Module: crate
// Provides: {"VecGrowScan"}
// Dependencies: {}
# [doc = " Forward scan over a vector with mutation, item insertion and removal."] # [doc = ""] # [doc = " Provides an iterator like interface over a vector which allows mutation,"] # [doc = " inserting new items before or after the current items, and removal of items."] # [doc = ""] # [doc = " If you do not need to insert new items, use [`VecMutScan`] instead."] # [doc = ""] # [doc = " Internally, the items are kept in the vector in order. When items are removed, a gap of"] # [doc = " uninitialized memory is created, and the items get moved as the iteration continues. When"] # [doc = " additional items are inserted, and there is no gap to be filled, the excess is stored in a"] # [doc = " [`VecDeque`]."] # [doc = ""] # [doc = " Overall, a linear number of moves is performed, but the exact number varies due to potential"] # [doc = " reallocations. If no items are inserted, every item is moved at most once."] # [doc = ""] # [doc = " Dropping the `VecGrowScan` mid-iteration keeps remaining items in the vector."] # [doc = ""] # [doc = " This does not implement the iterator trait, as the returned items borrow from this (i.e. this is"] # [doc = " a streaming iterator)."] # [doc = ""] # [doc = " The [`next`](VecGrowScan::next) method returns [`VecGrowScanItem`] values, which auto dereference"] # [doc = " to the vector's item type but also provide a [`remove`](VecGrowScanItem::remove) and"] # [doc = " [`replace`](VecGrowScanItem::replace) method."] pub struct VecGrowScan < 'a , T : 'a > { vec : & 'a mut Vec < T > , base : * mut T , write : usize , read : usize , end : usize , queue : VecDeque < T > , }
};
}

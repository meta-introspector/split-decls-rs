// Generated macro for UniqueItem (struct)
macro_rules! Depcrate_algorithms_utilsUniqueItem {
() => {
// Module: crate::algorithms::utils
// Provides: {"UniqueItem"}
// Dependencies: {}
# [doc = " Represents an item in the vector returned by [`unique`]."] # [doc = ""] # [doc = " It compares like the underlying item does it was created from but"] # [doc = " carries the index it was originally created from."] pub struct UniqueItem < 'a , Idx : ? Sized > { lookup : & 'a Idx , index : usize , }
};
}

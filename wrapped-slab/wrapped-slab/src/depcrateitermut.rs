// Generated macro for IterMut (struct)
macro_rules! DepcrateIterMut {
() => {
// Module: crate
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the values stored in the `Slab`"] pub struct IterMut < 'a , T > { entries : iter :: Enumerate < slice :: IterMut < 'a , Entry < T > > > , len : usize , }
};
}

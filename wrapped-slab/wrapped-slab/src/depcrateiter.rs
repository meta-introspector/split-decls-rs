// Generated macro for Iter (struct)
macro_rules! DepcrateIter {
() => {
// Module: crate
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the values stored in the `Slab`"] pub struct Iter < 'a , T > { entries : iter :: Enumerate < slice :: Iter < 'a , Entry < T > > > , len : usize , }
};
}

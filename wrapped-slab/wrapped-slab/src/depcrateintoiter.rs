// Generated macro for IntoIter (struct)
macro_rules! DepcrateIntoIter {
() => {
// Module: crate
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " A consuming iterator over the values stored in a `Slab`"] pub struct IntoIter < T > { entries : iter :: Enumerate < vec :: IntoIter < Entry < T > > > , len : usize , }
};
}

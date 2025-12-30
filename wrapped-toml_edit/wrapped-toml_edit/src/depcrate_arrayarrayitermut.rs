// Generated macro for ArrayIterMut (type)
macro_rules! Depcrate_arrayArrayIterMut {
() => {
// Module: crate::array
// Provides: {"ArrayIterMut"}
// Dependencies: {}
# [doc = " An iterator type over [`Array`]'s [`Value`]s"] pub type ArrayIterMut < 'a > = Box < dyn Iterator < Item = & 'a mut Value > + 'a > ;
};
}

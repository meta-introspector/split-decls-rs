// Generated macro for ArrayIter (type)
macro_rules! Depcrate_arrayArrayIter {
() => {
// Module: crate::array
// Provides: {"ArrayIter"}
// Dependencies: {}
# [doc = " An iterator type over [`Array`]'s [`Value`]s"] pub type ArrayIter < 'a > = Box < dyn Iterator < Item = & 'a Value > + 'a > ;
};
}

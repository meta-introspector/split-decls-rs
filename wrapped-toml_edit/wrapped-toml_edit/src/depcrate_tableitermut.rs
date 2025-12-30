// Generated macro for IterMut (type)
macro_rules! Depcrate_tableIterMut {
() => {
// Module: crate::table
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator type over [`Table`]'s [`Key`]/[`Item`] pairs"] pub type IterMut < 'a > = Box < dyn Iterator < Item = (KeyMut < 'a > , & 'a mut Item) > + 'a > ;
};
}

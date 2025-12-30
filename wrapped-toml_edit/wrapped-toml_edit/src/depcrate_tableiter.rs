// Generated macro for Iter (type)
macro_rules! Depcrate_tableIter {
() => {
// Module: crate::table
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator type over [`Table`]'s [`Key`]/[`Item`] pairs"] pub type Iter < 'a > = Box < dyn Iterator < Item = (& 'a str , & 'a Item) > + 'a > ;
};
}

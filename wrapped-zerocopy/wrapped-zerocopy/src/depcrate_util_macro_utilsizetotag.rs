// Generated macro for SizeToTag (type)
macro_rules! Depcrate_util_macro_utilSizeToTag {
() => {
// Module: crate::util::macro_util
// Provides: {"SizeToTag"}
// Dependencies: {}
# [doc = " An alias for the unsigned integer of the given size in bytes."] # [doc (hidden)] pub type SizeToTag < const SIZE : usize > = < () as size_to_tag :: SizeToTag < SIZE > > :: Tag ;
};
}

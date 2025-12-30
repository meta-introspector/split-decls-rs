// Generated macro for size_to_tag (module)
macro_rules! Depcrate_util_macro_utilsize_to_tag {
() => {
// Module: crate::util::macro_util
// Provides: {"size_to_tag"}
// Dependencies: {}
mod size_to_tag { pub trait SizeToTag < const SIZE : usize > { type Tag ; } impl SizeToTag < 1 > for () { type Tag = u8 ; } impl SizeToTag < 2 > for () { type Tag = u16 ; } impl SizeToTag < 4 > for () { type Tag = u32 ; } impl SizeToTag < 8 > for () { type Tag = u64 ; } impl SizeToTag < 16 > for () { type Tag = u128 ; } }
};
}

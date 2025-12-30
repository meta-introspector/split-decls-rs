// Generated macro for impl_80 (impl)
macro_rules! Depcrate_builder_nonconst_storeimpl_80 {
() => {
// Module: crate::builder::nonconst::store
// Provides: {"impl_80"}
// Dependencies: {}
impl NonConstLengthsStack { # [doc = " Creates a new empty [`NonConstLengthsStack`]."] pub const fn new () -> Self { Self { data : Vec :: new () } } # [doc = " Returns whether the stack is empty."] pub fn is_empty (& self) -> bool { self . data . is_empty () } # [doc = " Adds a [`BranchMeta`] to the stack."] pub fn push (& mut self , meta : BranchMeta) { self . data . push (meta) ; } # [doc = " Returns a copy of the [`BranchMeta`] on the top of the stack, panicking if"] # [doc = " the stack is empty."] # [allow (clippy :: unwrap_used)] pub fn peek_or_panic (& self) -> BranchMeta { * self . data . last () . unwrap () } # [doc = " Removes many [`BranchMeta`]s from the stack, returning them in a [`ConstArrayBuilder`]."] pub fn pop_many_or_panic (& mut self , len : usize) -> ConstArrayBuilder < 256 , BranchMeta > { debug_assert ! (len <= 256) ; let mut result = ConstArrayBuilder :: new_empty ([BranchMeta :: default () ; 256] , 256) ; let mut ix = 0 ; loop { if ix == len { break ; } let i = self . data . len () - ix - 1 ; result = result . const_push_front_or_panic (match self . data . get (i) { Some (x) => * x , None => unreachable ! ("Not enough items in the ConstLengthsStack") , }) ; ix += 1 ; } self . data . truncate (self . data . len () - len) ; result } # [doc = " Non-const function that returns the initialized elements as a slice."] fn as_slice (& self) -> & [BranchMeta] { & self . data } }
};
}

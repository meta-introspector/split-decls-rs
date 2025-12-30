// Generated macro for IterMut (struct)
macro_rules! Depcrate_bagIterMut {
() => {
// Module: crate::bag
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the entries of a [`Bag`]."] # [derive (Debug)] pub struct IterMut < 'b , T , const ARRAY_LEN : usize = DEFAULT_ARRAY_LEN > { bag : & 'b mut Bag < T , ARRAY_LEN > , current_index : u32 , current_stack_entry : Option < & 'b mut LinkedEntry < Storage < T , ARRAY_LEN > > > , }
};
}

// Generated macro for Serializer (struct)
macro_rules! Depcrate_serSerializer {
() => {
// Module: crate::ser
// Provides: {"Serializer"}
// Dependencies: {}
# [doc = " Serializer adapter that avoids stack overflows by dynamically growing the"] # [doc = " stack."] # [doc = ""] # [doc = " At each level of nested serialization, the adapter will check whether it is"] # [doc = " within `red_zone` bytes of the end of the stack. If so, it will allocate a"] # [doc = " new stack of size `stack_size` on which to continue deserialization."] pub struct Serializer < S > { pub ser : S , pub red_zone : usize , pub stack_size : usize , }
};
}

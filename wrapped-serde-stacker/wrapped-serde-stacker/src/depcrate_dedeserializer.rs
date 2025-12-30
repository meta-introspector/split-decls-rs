// Generated macro for Deserializer (struct)
macro_rules! Depcrate_deDeserializer {
() => {
// Module: crate::de
// Provides: {"Deserializer"}
// Dependencies: {}
# [doc = " Deserializer adapter that avoids stack overflows by dynamically growing the"] # [doc = " stack."] # [doc = ""] # [doc = " At each level of nested deserialization, the adapter will check whether it"] # [doc = " is within `red_zone` bytes of the end of the stack. If so, it will allocate"] # [doc = " a new stack of size `stack_size` on which to continue deserialization."] pub struct Deserializer < D > { pub de : D , pub red_zone : usize , pub stack_size : usize , }
};
}

// Generated macro for impl_736 (impl)
macro_rules! Depcrate_parseimpl_736 {
() => {
// Module: crate::parse
// Provides: {"impl_736"}
// Dependencies: {}
impl < 'c , 'a > StepCursor < 'c , 'a > { # [doc = " Triggers an error at the current position of the parse stream."] # [doc = ""] # [doc = " The `ParseStream::step` invocation will return this same error without"] # [doc = " advancing the stream state."] pub fn error < T : Display > (self , message : T) -> Error { error :: new_at (self . scope , self . cursor , message) } }
};
}

// Generated macro for CallStack (struct)
macro_rules! Depcrate_renderer_call_stackCallStack {
() => {
// Module: crate::renderer::call_stack
// Provides: {"CallStack"}
// Dependencies: {}
# [doc = " Contains the stack of frames"] # [derive (Debug)] pub struct CallStack < 'a > { # [doc = " The stack of frames"] stack : Vec < StackFrame < 'a > > , # [doc = " User supplied context for the render"] context : UserContext < 'a > , }
};
}

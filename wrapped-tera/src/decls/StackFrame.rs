macro_rules! deps {
    () => {
        ForLoop!();
        FrameContext!();
        Template!();
        FrameType!();
    };
}

macro_rules! StackFrame {
    () => {
        deps!();
        # [doc = " Entry in the stack frame"] # [derive (Debug)] pub struct StackFrame < 'a > { # [doc = " Type of stack frame"] pub kind : FrameType , # [doc = " Frame name for context/debugging"] pub name : & 'a str , # [doc = " Assigned value (via {% set ... %}, {% for ... %}, {% namespace::macro(a=a, b=b) %})"] # [doc = ""] # [doc = " - {% set ... %} adds to current frame_context"] # [doc = " - {% for ... %} builds frame_context before iteration"] # [doc = " - {% namespace::macro(a=a, b=b)} builds frame_context before invocation"] context : FrameContext < 'a > , # [doc = " Active template for frame"] pub active_template : & 'a Template , # [doc = " `ForLoop` if frame is for a for loop"] pub for_loop : Option < ForLoop < 'a > > , # [doc = " Macro namespace if MacroFrame"] pub macro_namespace : Option < & 'a str > , }
    };
}

StackFrame!()
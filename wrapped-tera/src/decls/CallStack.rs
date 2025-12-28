macro_rules! deps {
    () => {
        UserContext!();
        StackFrame!();
    };
}

macro_rules! CallStack {
    () => {
        deps!();
        # [doc = " Contains the stack of frames"] # [derive (Debug)] pub struct CallStack < 'a > { # [doc = " The stack of frames"] stack : Vec < StackFrame < 'a > > , # [doc = " User supplied context for the render"] context : UserContext < 'a > , }
    };
}

CallStack!()
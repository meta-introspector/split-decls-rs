macro_rules! deps {
    () => {
        Dispatch!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [doc = " The dispatch state of a thread."] # [cfg (feature = "std")] struct State { # [doc = " This thread's current default dispatcher."] default : RefCell < Option < Dispatch > > , # [doc = " Whether or not we can currently begin dispatching a trace event."] # [doc = ""] # [doc = " This is set to `false` when functions such as `enter`, `exit`, `event`,"] # [doc = " and `new_span` are called on this thread's default dispatcher, to"] # [doc = " prevent further trace events triggered inside those functions from"] # [doc = " creating an infinite recursion. When we finish handling a dispatch, this"] # [doc = " is set back to `true`."] can_enter : Cell < bool > , }
    };
}

State!()
macro_rules! deps {
    () => {
        UserDefinedFlags!();
        Signal!();
        Pid!();
    };
}

macro_rules! EventFilter {
    () => {
        deps!();
        # [doc = " The possible filters for a `kqueue`."] # [repr (i16)] # [non_exhaustive] pub enum EventFilter { # [doc = " A read filter."] Read (RawFd) , # [doc = " A write filter."] Write (RawFd) , # [doc = " An empty filter."] # [cfg (target_os = "freebsd")] Empty (RawFd) , # [doc = " A VNode filter."] Vnode { # [doc = " The file descriptor we looked for events in."] vnode : RawFd , # [doc = " The flags for this event."] flags : VnodeEvents , } , # [doc = " A process filter."] Proc { # [doc = " The process ID we waited on."] pid : Pid , # [doc = " The flags for this event."] flags : ProcessEvents , } , # [doc = " A signal filter."] Signal { # [doc = " The signal number we waited on."] signal : Signal , # [doc = " The number of times the signal has been received since the last"] # [doc = " call to kevent."] times : usize , } , # [doc = " A timer filter."] Timer { # [doc = " The identifier for this event."] ident : intptr_t , # [doc = " The duration for this event."] timer : Option < Duration > , } , # [doc = " A user filter."] # [cfg (any (apple , freebsdlike))] User { # [doc = " The identifier for this event."] ident : intptr_t , # [doc = " The flags for this event."] flags : UserFlags , # [doc = " The user-defined flags for this event."] user_flags : UserDefinedFlags , } , # [doc = " This filter is unknown."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Passing this into `Event::new()` will result in a panic."] Unknown , }
    };
}

EventFilter!();
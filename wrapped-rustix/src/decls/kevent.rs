macro_rules! deps {
    () => {
        Timespec!();
        Result!();
        Buffer!();
        Event!();
    };
}

macro_rules! kevent {
    () => {
        deps!();
        # [doc = " `kevent(kqueue, changelist, eventlist, timeout)`—Wait for events on a"] # [doc = " `kqueue`."] # [doc = ""] # [doc = " This is a wrapper around [`kevent_timespec`] which takes a `Duration`"] # [doc = " instead of a `Timespec` for the timemout value. `Timespec` has a signed"] # [doc = " `i64` seconds field; if converting `Duration` to `Timespec` overflows,"] # [doc = " `None` is passed as the timeout instead, such such a large timeout would"] # [doc = " be effectively infinite in practice."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The file descriptors referred to by the `Event` structs must be valid for"] # [doc = " the lifetime of the `kqueue` file descriptor."] pub unsafe fn kevent < Fd : AsFd , Buf : Buffer < Event > > (kqueue : Fd , changelist : & [Event] , eventlist : Buf , timeout : Option < Duration > ,) -> io :: Result < Buf :: Output > { let timeout = match timeout { Some (timeout) => match timeout . as_secs () . try_into () { Ok (tv_sec) => Some (Timespec { tv_sec , tv_nsec : timeout . subsec_nanos () as _ , }) , Err (_) => None , } , None => None , } ; kevent_timespec (kqueue , changelist , eventlist , timeout . as_ref ()) }
    };
}

kevent!();
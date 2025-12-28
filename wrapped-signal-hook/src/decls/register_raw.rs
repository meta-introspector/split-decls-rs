macro_rules! deps {
    () => {
        WakeFd!();
        WakeMethod!();
    };
}

macro_rules! register_raw {
    () => {
        deps!();
        # [doc = " Registers a write to a self-pipe whenever there's the signal."] # [doc = ""] # [doc = " In this case, the pipe is taken as the `RawFd`. It'll be closed on deregistration. Effectively,"] # [doc = " the function takes ownership of the file descriptor. This includes feeling free to set arbitrary"] # [doc = " flags on it, including file status flags (that are shared across file descriptors created by"] # [doc = " `dup`)."] # [doc = ""] # [doc = " Note that passing the wrong file descriptor won't cause UB, but can still lead to severe bugs ‒"] # [doc = " like data corruptions in files. Prefer using [`register`] if possible."] # [doc = ""] # [doc = " Also, it is perfectly legal for multiple writes to be collated together (if not consumed) and"] # [doc = " to generate spurious wakeups (but will not generate spurious *bytes* in the pipe)."] # [doc = ""] # [doc = " # Internal details"] # [doc = ""] # [doc = " Internally, it *currently* does following. Note that this is *not* part of the stability"] # [doc = " guarantees and may change if necessary."] # [doc = ""] # [doc = " * If the file descriptor can be used with [`send`][libc::send], it'll be used together with"] # [doc = "   [`MSG_DONTWAIT`][libc::MSG_DONTWAIT]. This is tested by sending `0` bytes of data (depending"] # [doc = "   on the socket type, this might wake the read end with an empty message)."] # [doc = " * If it is not possible, the [`O_NONBLOCK`][libc::O_NONBLOCK] will be set on the file"] # [doc = "   descriptor and [`write`][libc::write] will be used instead."] pub fn register_raw (signal : c_int , pipe : RawFd) -> Result < SigId , Error > { let res = unsafe { libc :: send (pipe , & [] as * const _ , 0 , MSG_NOWAIT) } ; let fd = match (res , Error :: last_os_error () . kind ()) { (0 , _) | (- 1 , ErrorKind :: WouldBlock) => WakeFd { fd : pipe , method : WakeMethod :: Send , } , _ => { let fd = WakeFd { fd : pipe , method : WakeMethod :: Write , } ; fd . set_flags () ? ; fd } } ; let action = move | | fd . wake () ; unsafe { super :: register (signal , action) } }
    };
}

register_raw!();
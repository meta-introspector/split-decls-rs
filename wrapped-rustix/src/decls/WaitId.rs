macro_rules! deps {
    () => {
        Pid!();
        Wait!();
    };
}

macro_rules! WaitId {
    () => {
        deps!();
        # [doc = " The identifier to wait on in a call to [`waitid`]."] # [cfg (not (any (target_os = "openbsd" , target_os = "redox" , target_os = "wasi")))] # [derive (Debug , Clone)] # [non_exhaustive] pub enum WaitId < 'a > { # [doc = " Wait on all processes."] # [doc (alias = "P_ALL")] All , # [doc = " Wait for a specific process ID."] # [doc (alias = "P_PID")] Pid (Pid) , # [doc = " Wait for a specific process group ID, or the calling process' group ID."] # [doc (alias = "P_PGID")] Pgid (Option < Pid >) , # [doc = " Wait for a specific process file descriptor."] # [cfg (target_os = "linux")] # [doc (alias = "P_PIDFD")] PidFd (BorrowedFd < 'a >) , # [doc = " Eat the lifetime for non-Linux platforms."] # [doc (hidden)] # [cfg (not (target_os = "linux"))] __EatLifetime (core :: marker :: PhantomData < & 'a () >) , }
    };
}

WaitId!()
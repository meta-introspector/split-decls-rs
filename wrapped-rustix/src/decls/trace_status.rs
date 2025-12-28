macro_rules! deps {
    () => {
        Result!();
        Pid!();
        TracingStatus!();
        RawPid!();
        ProcSelector!();
    };
}

macro_rules! trace_status {
    () => {
        deps!();
        # [doc = " Get the tracing status of the process indicated by `idtype` and `id`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [FreeBSD `procctl(PROC_TRACE_STATUS,…)`]"] # [doc = ""] # [doc = " [FreeBSD `procctl(PROC_TRACE_STATUS,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [inline] pub fn trace_status (process : ProcSelector) -> io :: Result < TracingStatus > { let val = unsafe { procctl_get_optional :: < c_int > (PROC_TRACE_STATUS , process) } ? ; match val { - 1 => Ok (TracingStatus :: NotTraceble) , 0 => Ok (TracingStatus :: Tracable) , pid => { let pid = Pid :: from_raw (pid as RawPid) . ok_or (io :: Errno :: RANGE) ? ; Ok (TracingStatus :: BeingTraced (pid)) } } }
    };
}

trace_status!()
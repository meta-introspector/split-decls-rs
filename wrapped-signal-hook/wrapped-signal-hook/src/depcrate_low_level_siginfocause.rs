// Generated macro for Cause (enum)
macro_rules! Depcrate_low_level_siginfoCause {
() => {
// Module: crate::low_level::siginfo
// Provides: {"Cause"}
// Dependencies: {}
# [doc = " What caused a signal."] # [doc = ""] # [doc = " This is a best-effort (and possibly incomplete) representation of the C `siginfo_t::si_code`."] # [doc = " It may differ between OSes and may be extended in future versions."] # [doc = ""] # [doc = " Note that this doesn't contain all the „fault“ signals (`SIGILL`, `SIGSEGV` and similar)."] # [doc = " There's no reasonable way to use the exfiltrators with them, since the handler either needs to"] # [doc = " terminate the process or somehow recover from the situation. Things based on exfiltrators do"] # [doc = " neither, which would cause an UB and therefore these values just don't make sense."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Cause { # [doc = " The cause is unknown."] # [doc = ""] # [doc = " Some systems don't fill this in. Some systems have values we don't understand. Some signals"] # [doc = " don't have specific reasons to come to being."] Unknown , # [doc = " Sent by the kernel."] # [doc = ""] # [doc = " This probably exists only on Linux."] Kernel , # [doc = " The signal was sent by other process."] Sent (Sent) , # [doc = " A `SIGCHLD`, caused by a child process changing state."] Chld (Chld) , }
};
}

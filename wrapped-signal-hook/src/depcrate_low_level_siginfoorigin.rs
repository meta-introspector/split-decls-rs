// Generated macro for Origin (struct)
macro_rules! Depcrate_low_level_siginfoOrigin {
() => {
// Module: crate::low_level::siginfo
// Provides: {"Origin"}
// Dependencies: {}
# [doc = " Information about a signal and its origin."] # [doc = ""] # [doc = " This is produced by the [`WithOrigin`] exfiltrator (or can be [extracted][Origin::extract] from"] # [doc = " `siginfo_t` by hand)."] # [derive (Clone , Eq , PartialEq)] # [non_exhaustive] pub struct Origin { # [doc = " The signal that happened."] pub signal : c_int , # [doc = " Information about the process that caused the signal."] # [doc = ""] # [doc = " Note that not all signals are caused by a specific process or have the information"] # [doc = " available („fault“ signals like `SIGBUS` don't have, any signal may be sent by the kernel"] # [doc = " instead of a specific process)."] # [doc = ""] # [doc = " This is filled in whenever available. For most signals, this is the process that sent the"] # [doc = " signal (by `kill` or similar), for `SIGCHLD` it is the child that caused the signal."] pub process : Option < Process > , # [doc = " How the signal happened."] # [doc = ""] # [doc = " This is a best-effort value. In particular, some systems may have causes not known to this"] # [doc = " library. Some other systems (MacOS) does not fill the value in so there's no way to know."] # [doc = " In all these cases, this will contain [`Cause::Unknown`]."] # [doc = ""] # [doc = " Some values are platform specific and not available on other systems."] # [doc = ""] # [doc = " Future versions may enrich the enum by further values."] pub cause : Cause , }
};
}

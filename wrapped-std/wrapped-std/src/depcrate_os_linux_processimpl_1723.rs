// Generated macro for impl_1723 (impl)
macro_rules! Depcrate_os_linux_processimpl_1723 {
() => {
// Module: crate::os::linux::process
// Provides: {"impl_1723"}
// Dependencies: {}
impl PidFd { # [doc = " Forces the child process to exit."] # [doc = ""] # [doc = " Unlike [`Child::kill`] it is possible to attempt to kill"] # [doc = " reaped children since PidFd does not suffer from pid recycling"] # [doc = " races. But doing so will return an Error."] # [doc = ""] # [doc = " [`Child::kill`]: process::Child::kill"] pub fn kill (& self) -> Result < () > { self . inner . kill () } # [doc = " Waits for the child to exit completely, returning the status that it exited with."] # [doc = ""] # [doc = " Unlike [`Child::wait`] it does not ensure that the stdin handle is closed."] # [doc = " Additionally it will not return an `ExitStatus` if the child"] # [doc = " has already been reaped. Instead an error will be returned."] # [doc = ""] # [doc = " [`Child::wait`]: process::Child::wait"] pub fn wait (& self) -> Result < ExitStatus > { self . inner . wait () . map (FromInner :: from_inner) } # [doc = " Attempts to collect the exit status of the child if it has already exited."] # [doc = ""] # [doc = " Unlike [`Child::try_wait`] this method will return an Error"] # [doc = " if the child has already been reaped."] # [doc = ""] # [doc = " [`Child::try_wait`]: process::Child::try_wait"] pub fn try_wait (& self) -> Result < Option < ExitStatus > > { Ok (self . inner . try_wait () ? . map (FromInner :: from_inner)) } }
};
}

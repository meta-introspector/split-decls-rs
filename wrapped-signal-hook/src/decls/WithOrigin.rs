macro_rules! deps {
    () => {
        WithRawSiginfo!();
        Origin!();
    };
}

macro_rules! WithOrigin {
    () => {
        deps!();
        # [doc = " The [`Exfiltrator`][crate::iterator::exfiltrator::Exfiltrator] that produces [`Origin`] of"] # [doc = " signals."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use signal_hook::consts::SIGUSR1;"] # [doc = " # use signal_hook::iterator::SignalsInfo;"] # [doc = " # use signal_hook::iterator::exfiltrator::WithOrigin;"] # [doc = " #"] # [doc = " # fn main() -> Result<(), std::io::Error> {"] # [doc = " // Subscribe to SIGUSR1, with information about the process."] # [doc = " let mut signals = SignalsInfo::<WithOrigin>::new(&[SIGUSR1])?;"] # [doc = ""] # [doc = " // Send a signal to ourselves."] # [doc = " let my_pid = unsafe { libc::getpid() };"] # [doc = " unsafe { libc::kill(my_pid, SIGUSR1) };"] # [doc = ""] # [doc = " // Grab the signal and look into the details."] # [doc = " let received = signals.wait().next().unwrap();"] # [doc = ""] # [doc = " assert_eq!(SIGUSR1, received.signal);"] # [doc = " assert_eq!(my_pid, received.process.unwrap().pid);"] # [doc = " # Ok(()) }"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Default)] pub struct WithOrigin (WithRawSiginfo) ;
    };
}

WithOrigin!();
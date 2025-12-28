macro_rules! deps {
    () => {
        Termios!();
    };
}

macro_rules! macro_1201 {
    () => {
        deps!();
        bitflags ! { # [doc = " Flags controlling “local” terminal modes."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct LocalModes : types :: tcflag_t { # [doc = " `XCASE`"] # [cfg (any (linux_raw_dep , target_arch = "s390x" , target_os = "haiku"))] const XCASE = c :: XCASE ; # [doc = " `ECHOCTL`"] # [cfg (not (target_os = "redox"))] const ECHOCTL = c :: ECHOCTL ; # [doc = " `ECHOPRT`"] # [cfg (not (any (target_os = "cygwin" , target_os = "nto" , target_os = "redox")))] const ECHOPRT = c :: ECHOPRT ; # [doc = " `ECHOKE`"] # [cfg (not (target_os = "redox"))] const ECHOKE = c :: ECHOKE ; # [doc = " `FLUSHO`"] # [cfg (not (any (target_os = "nto" , target_os = "redox")))] const FLUSHO = c :: FLUSHO ; # [doc = " `PENDIN`"] # [cfg (not (any (target_os = "cygwin" , target_os = "nto" , target_os = "redox")))] const PENDIN = c :: PENDIN ; # [doc = " `EXTPROC`"] # [cfg (not (any (target_os = "aix" , target_os = "cygwin" , target_os = "haiku" , target_os = "nto" , target_os = "redox" ,)))] const EXTPROC = c :: EXTPROC ; # [doc = " `ISIG`"] const ISIG = c :: ISIG ; # [doc = " `ICANON`—A flag for the `c_lflag` field of [`Termios`] indicating"] # [doc = " canonical mode."] const ICANON = c :: ICANON ; # [doc = " `ECHO`"] const ECHO = c :: ECHO ; # [doc = " `ECHOE`"] const ECHOE = c :: ECHOE ; # [doc = " `ECHOK`"] const ECHOK = c :: ECHOK ; # [doc = " `ECHONL`"] const ECHONL = c :: ECHONL ; # [doc = " `NOFLSH`"] const NOFLSH = c :: NOFLSH ; # [doc = " `TOSTOP`"] const TOSTOP = c :: TOSTOP ; # [doc = " `IEXTEN`"] const IEXTEN = c :: IEXTEN ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_1201!()
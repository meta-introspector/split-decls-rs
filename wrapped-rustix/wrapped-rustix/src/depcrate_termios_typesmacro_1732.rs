// Generated macro for macro_1732 (macro)
macro_rules! Depcrate_termios_typesmacro_1732 {
() => {
// Module: crate::termios::types
// Provides: {"macro_1732"}
// Dependencies: {}
bitflags ! { # [doc = " Flags controlling special terminal modes."] # [doc = ""] # [doc = " `CBAUD`, `CBAUDEX`, `CIBAUD`, `CIBAUDEX`, and various `B*` speed"] # [doc = " constants are often included in the control modes, however rustix"] # [doc = " handles them separately, in [`Termios::set_speed`] and related"] # [doc = " functions. If you see extra bits in the `Debug` output, they're"] # [doc = " probably these flags."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct ControlModes : types :: tcflag_t { # [doc = " `CSIZE`"] const CSIZE = c :: CSIZE ; # [doc = " `CS5`"] const CS5 = c :: CS5 ; # [doc = " `CS6`"] const CS6 = c :: CS6 ; # [doc = " `CS7`"] const CS7 = c :: CS7 ; # [doc = " `CS8`"] const CS8 = c :: CS8 ; # [doc = " `CSTOPB`"] const CSTOPB = c :: CSTOPB ; # [doc = " `CREAD`"] const CREAD = c :: CREAD ; # [doc = " `PARENB`"] const PARENB = c :: PARENB ; # [doc = " `PARODD`"] const PARODD = c :: PARODD ; # [doc = " `HUPCL`"] const HUPCL = c :: HUPCL ; # [doc = " `CLOCAL`"] const CLOCAL = c :: CLOCAL ; # [doc = " `CRTSCTS`"] # [cfg (not (any (target_os = "aix" , target_os = "nto" , target_os = "redox")))] const CRTSCTS = c :: CRTSCTS ; # [doc = " `CMSPAR`"] # [cfg (not (any (bsd , solarish , target_os = "aix" , target_os = "emscripten" , target_os = "haiku" , target_os = "hurd" , target_os = "nto" , target_os = "redox" ,)))] const CMSPAR = c :: CMSPAR ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}

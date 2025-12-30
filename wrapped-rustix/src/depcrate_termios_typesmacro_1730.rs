// Generated macro for macro_1730 (macro)
macro_rules! Depcrate_termios_typesmacro_1730 {
() => {
// Module: crate::termios::types
// Provides: {"macro_1730"}
// Dependencies: {}
bitflags ! { # [doc = " Flags controlling terminal input."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct InputModes : types :: tcflag_t { # [doc = " `IGNBRK`"] const IGNBRK = c :: IGNBRK ; # [doc = " `BRKINT`"] const BRKINT = c :: BRKINT ; # [doc = " `IGNPAR`"] const IGNPAR = c :: IGNPAR ; # [doc = " `PARMRK`"] const PARMRK = c :: PARMRK ; # [doc = " `INPCK`"] const INPCK = c :: INPCK ; # [doc = " `ISTRIP`"] const ISTRIP = c :: ISTRIP ; # [doc = " `INLCR`"] const INLCR = c :: INLCR ; # [doc = " `IGNCR`"] const IGNCR = c :: IGNCR ; # [doc = " `ICRNL`"] const ICRNL = c :: ICRNL ; # [doc = " `IUCLC`"] # [cfg (any (linux_raw_dep , solarish , target_os = "aix" , target_os = "haiku" , target_os = "nto"))] const IUCLC = c :: IUCLC ; # [doc = " `IXON`"] const IXON = c :: IXON ; # [doc = " `IXANY`"] # [cfg (not (target_os = "redox"))] const IXANY = c :: IXANY ; # [doc = " `IXOFF`"] const IXOFF = c :: IXOFF ; # [doc = " `IMAXBEL`"] # [cfg (not (any (target_os = "haiku" , target_os = "redox")))] const IMAXBEL = c :: IMAXBEL ; # [doc = " `IUTF8`"] # [cfg (not (any (freebsdlike , netbsdlike , solarish , target_os = "aix" , target_os = "emscripten" , target_os = "haiku" , target_os = "hurd" , target_os = "redox" ,)))] const IUTF8 = c :: IUTF8 ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}

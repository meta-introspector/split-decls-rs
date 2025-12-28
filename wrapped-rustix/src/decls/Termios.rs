macro_rules! deps {
    () => {
        SpecialCodes!();
        How!();
    };
}

macro_rules! Termios {
    () => {
        deps!();
        # [doc = " `struct termios` for use with [`tcgetattr`] and [`tcsetattr`]."] # [doc = ""] # [doc = " [`tcgetattr`]: crate::termios::tcgetattr"] # [doc = " [`tcsetattr`]: crate::termios::tcsetattr"] # [repr (C)] # [derive (Clone)] pub struct Termios { # [doc = " How is input interpreted?"] # [doc (alias = "c_iflag")] pub input_modes : InputModes , # [doc = " How is output translated?"] # [doc (alias = "c_oflag")] pub output_modes : OutputModes , # [doc = " Low-level configuration flags."] # [doc (alias = "c_cflag")] pub control_modes : ControlModes , # [doc = " High-level configuration flags."] # [doc (alias = "c_lflag")] pub local_modes : LocalModes , # [doc = " Line discipline."] # [doc (alias = "c_line")] # [cfg (not (all (linux_raw , any (target_arch = "powerpc" , target_arch = "powerpc64"))))] # [cfg (any (linux_like , target_env = "newlib" , target_os = "fuchsia" , target_os = "haiku" , target_os = "redox"))] pub line_discipline : u8 , # [doc = " How are various special control codes handled?"] # [doc (alias = "c_cc")] # [cfg (not (target_os = "haiku"))] pub special_codes : SpecialCodes , # [cfg (target_os = "nto")] pub (crate) __reserved : [ffi :: c_uint ; 3] , # [doc = " Line discipline."] # [doc (alias = "c_line")] # [cfg (all (linux_raw , any (target_arch = "powerpc" , target_arch = "powerpc64")))] pub line_discipline : c :: cc_t , # [doc = " See the `input_speed` and `set_input_seed` functions."] # [doc = ""] # [doc = " On Linux and BSDs, this is the arbitrary integer speed value. On all"] # [doc = " other platforms, this is the encoded speed value."] # [cfg (not (any (solarish , all (libc , target_env = "newlib") , target_os = "aix")))] pub (crate) input_speed : c :: speed_t , # [doc = " See the `output_speed` and `set_output_seed` functions."] # [doc = ""] # [doc = " On Linux and BSDs, this is the integer speed value. On all other"] # [doc = " platforms, this is the encoded speed value."] # [cfg (not (any (solarish , all (libc , target_env = "newlib") , target_os = "aix")))] pub (crate) output_speed : c :: speed_t , # [doc = " How are various special control codes handled?"] # [doc (alias = "c_cc")] # [cfg (target_os = "haiku")] pub special_codes : SpecialCodes , }
    };
}

Termios!();
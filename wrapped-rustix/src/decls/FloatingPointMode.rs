macro_rules! FloatingPointMode {
    () => {
        # [doc = " `PR_FP_MODE_*` values for use with [`floating_point_mode`] and"] # [doc = " [`set_floating_point_mode`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u32)] pub enum FloatingPointMode { # [doc = " 64-bit floating point registers."] FloatingPointRegisters = PR_FP_MODE_FR , # [doc = " Enable emulation of 32-bit floating-point mode."] FloatingPointEmulation = PR_FP_MODE_FRE , }
    };
}

FloatingPointMode!();
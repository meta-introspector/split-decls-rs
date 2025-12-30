// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f405_fpuRegisterBlock {
() => {
// Module: crate::stm32f405::fpu
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Floating-point context control register"] pub fpccr : crate :: Reg < fpccr :: FPCCR_SPEC > , # [doc = "0x04 - Floating-point context address register"] pub fpcar : crate :: Reg < fpcar :: FPCAR_SPEC > , # [doc = "0x08 - Floating-point status control register"] pub fpscr : crate :: Reg < fpscr :: FPSCR_SPEC > , }
};
}

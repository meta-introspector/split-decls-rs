// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f405_rngRegisterBlock {
() => {
// Module: crate::stm32f405::rng
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - control register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x04 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x08 - data register"] pub dr : crate :: Reg < dr :: DR_SPEC > , }
};
}

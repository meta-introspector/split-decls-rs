// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f413_wwdgRegisterBlock {
() => {
// Module: crate::stm32f413::wwdg
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Control register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x04 - Configuration register"] pub cfr : crate :: Reg < cfr :: CFR_SPEC > , # [doc = "0x08 - Status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , }
};
}

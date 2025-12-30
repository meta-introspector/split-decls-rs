// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f413_pwrRegisterBlock {
() => {
// Module: crate::stm32f413::pwr
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - power control register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x04 - power control/status register"] pub csr : crate :: Reg < csr :: CSR_SPEC > , }
};
}

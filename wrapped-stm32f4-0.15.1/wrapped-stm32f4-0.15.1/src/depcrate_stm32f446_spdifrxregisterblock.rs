// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f446_spdifrxRegisterBlock {
() => {
// Module: crate::stm32f446::spdifrx
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Control register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x04 - Interrupt mask register"] pub imr : crate :: Reg < imr :: IMR_SPEC > , # [doc = "0x08 - Status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x0c - Interrupt Flag Clear register"] pub ifcr : crate :: Reg < ifcr :: IFCR_SPEC > , # [doc = "0x10 - Data input register"] pub dr : crate :: Reg < dr :: DR_SPEC > , # [doc = "0x14 - Channel Status register"] pub csr : crate :: Reg < csr :: CSR_SPEC > , # [doc = "0x18 - Debug Information register"] pub dir : crate :: Reg < dir :: DIR_SPEC > , }
};
}

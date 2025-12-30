// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f413_lptimRegisterBlock {
() => {
// Module: crate::stm32f413::lptim
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Interrupt and Status Register"] pub isr : crate :: Reg < isr :: ISR_SPEC > , # [doc = "0x04 - Interrupt Clear Register"] pub icr : crate :: Reg < icr :: ICR_SPEC > , # [doc = "0x08 - Interrupt Enable Register"] pub ier : crate :: Reg < ier :: IER_SPEC > , # [doc = "0x0c - Configuration Register"] pub cfgr : crate :: Reg < cfgr :: CFGR_SPEC > , # [doc = "0x10 - Control Register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x14 - Compare Register"] pub cmp : crate :: Reg < cmp :: CMP_SPEC > , # [doc = "0x18 - Autoreload Register"] pub arr : crate :: Reg < arr :: ARR_SPEC > , # [doc = "0x1c - Counter Register"] pub cnt : crate :: Reg < cnt :: CNT_SPEC > , }
};
}

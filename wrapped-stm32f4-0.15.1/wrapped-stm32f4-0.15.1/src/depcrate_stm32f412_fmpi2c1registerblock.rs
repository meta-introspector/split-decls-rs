// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f412_fmpi2c1RegisterBlock {
() => {
// Module: crate::stm32f412::fmpi2c1
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Control register 1"] pub cr1 : crate :: Reg < cr1 :: CR1_SPEC > , # [doc = "0x04 - Control register 2"] pub cr2 : crate :: Reg < cr2 :: CR2_SPEC > , # [doc = "0x08 - Own address register 1"] pub oar1 : crate :: Reg < oar1 :: OAR1_SPEC > , # [doc = "0x0c - Own address register 2"] pub oar2 : crate :: Reg < oar2 :: OAR2_SPEC > , # [doc = "0x10 - Timing register"] pub timingr : crate :: Reg < timingr :: TIMINGR_SPEC > , # [doc = "0x14 - Timeout register"] pub timeoutr : crate :: Reg < timeoutr :: TIMEOUTR_SPEC > , # [doc = "0x18 - Interrupt and Status register"] pub isr : crate :: Reg < isr :: ISR_SPEC > , # [doc = "0x1c - Interrupt clear register"] pub icr : crate :: Reg < icr :: ICR_SPEC > , # [doc = "0x20 - PEC register"] pub pecr : crate :: Reg < pecr :: PECR_SPEC > , # [doc = "0x24 - Receive data register"] pub rxdr : crate :: Reg < rxdr :: RXDR_SPEC > , # [doc = "0x28 - Transmit data register"] pub txdr : crate :: Reg < txdr :: TXDR_SPEC > , }
};
}

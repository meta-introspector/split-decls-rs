// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f405_i2c1RegisterBlock {
() => {
// Module: crate::stm32f405::i2c1
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Control register 1"] pub cr1 : crate :: Reg < cr1 :: CR1_SPEC > , # [doc = "0x04 - Control register 2"] pub cr2 : crate :: Reg < cr2 :: CR2_SPEC > , # [doc = "0x08 - Own address register 1"] pub oar1 : crate :: Reg < oar1 :: OAR1_SPEC > , # [doc = "0x0c - Own address register 2"] pub oar2 : crate :: Reg < oar2 :: OAR2_SPEC > , # [doc = "0x10 - Data register"] pub dr : crate :: Reg < dr :: DR_SPEC > , # [doc = "0x14 - Status register 1"] pub sr1 : crate :: Reg < sr1 :: SR1_SPEC > , # [doc = "0x18 - Status register 2"] pub sr2 : crate :: Reg < sr2 :: SR2_SPEC > , # [doc = "0x1c - Clock control register"] pub ccr : crate :: Reg < ccr :: CCR_SPEC > , # [doc = "0x20 - TRISE register"] pub trise : crate :: Reg < trise :: TRISE_SPEC > , }
};
}

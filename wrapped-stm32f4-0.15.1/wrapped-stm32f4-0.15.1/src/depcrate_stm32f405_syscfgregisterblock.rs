// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f405_syscfgRegisterBlock {
() => {
// Module: crate::stm32f405::syscfg
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - memory remap register"] pub memrm : crate :: Reg < memrm :: MEMRM_SPEC > , # [doc = "0x04 - peripheral mode configuration register"] pub pmc : crate :: Reg < pmc :: PMC_SPEC > , # [doc = "0x08 - external interrupt configuration register 1"] pub exticr1 : crate :: Reg < exticr1 :: EXTICR1_SPEC > , # [doc = "0x0c - external interrupt configuration register 2"] pub exticr2 : crate :: Reg < exticr2 :: EXTICR2_SPEC > , # [doc = "0x10 - external interrupt configuration register 3"] pub exticr3 : crate :: Reg < exticr3 :: EXTICR3_SPEC > , # [doc = "0x14 - external interrupt configuration register 4"] pub exticr4 : crate :: Reg < exticr4 :: EXTICR4_SPEC > , _reserved6 : [u8 ; 0x08] , # [doc = "0x20 - Compensation cell control register"] pub cmpcr : crate :: Reg < cmpcr :: CMPCR_SPEC > , }
};
}

// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f407_tim9RegisterBlock {
() => {
// Module: crate::stm32f407::tim9
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - control register 1"] pub cr1 : crate :: Reg < cr1 :: CR1_SPEC > , # [doc = "0x04 - control register 2"] pub cr2 : crate :: Reg < cr2 :: CR2_SPEC > , # [doc = "0x08 - slave mode control register"] pub smcr : crate :: Reg < smcr :: SMCR_SPEC > , # [doc = "0x0c - DMA/Interrupt enable register"] pub dier : crate :: Reg < dier :: DIER_SPEC > , # [doc = "0x10 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x14 - event generation register"] pub egr : crate :: Reg < egr :: EGR_SPEC > , _reserved_6_ccmr1 : [u8 ; 0x04] , _reserved7 : [u8 ; 0x04] , # [doc = "0x20 - capture/compare enable register"] pub ccer : crate :: Reg < ccer :: CCER_SPEC > , # [doc = "0x24 - counter"] pub cnt : crate :: Reg < cnt :: CNT_SPEC > , # [doc = "0x28 - prescaler"] pub psc : crate :: Reg < psc :: PSC_SPEC > , # [doc = "0x2c - auto-reload register"] pub arr : crate :: Reg < arr :: ARR_SPEC > , _reserved11 : [u8 ; 0x04] , # [doc = "0x34..0x3c - capture/compare register"] pub ccr : [crate :: Reg < ccr :: CCR_SPEC > ; 2] , }
};
}

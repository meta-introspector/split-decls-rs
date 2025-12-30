// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f407_tim11RegisterBlock {
() => {
// Module: crate::stm32f407::tim11
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - control register 1"] pub cr1 : crate :: Reg < cr1 :: CR1_SPEC > , _reserved1 : [u8 ; 0x08] , # [doc = "0x0c - DMA/Interrupt enable register"] pub dier : crate :: Reg < dier :: DIER_SPEC > , # [doc = "0x10 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x14 - event generation register"] pub egr : crate :: Reg < egr :: EGR_SPEC > , _reserved_4_ccmr1 : [u8 ; 0x04] , _reserved5 : [u8 ; 0x04] , # [doc = "0x20 - capture/compare enable register"] pub ccer : crate :: Reg < ccer :: CCER_SPEC > , # [doc = "0x24 - counter"] pub cnt : crate :: Reg < cnt :: CNT_SPEC > , # [doc = "0x28 - prescaler"] pub psc : crate :: Reg < psc :: PSC_SPEC > , # [doc = "0x2c - auto-reload register"] pub arr : crate :: Reg < arr :: ARR_SPEC > , _reserved9 : [u8 ; 0x04] , # [doc = "0x34 - capture/compare register"] pub ccr : [crate :: Reg < ccr :: CCR_SPEC > ; 1] , _reserved10 : [u8 ; 0x18] , # [doc = "0x50 - option register"] pub or : crate :: Reg < or :: OR_SPEC > , }
};
}

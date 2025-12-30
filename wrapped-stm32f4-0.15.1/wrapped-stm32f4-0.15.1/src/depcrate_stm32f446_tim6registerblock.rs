// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f446_tim6RegisterBlock {
() => {
// Module: crate::stm32f446::tim6
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - control register 1"] pub cr1 : crate :: Reg < cr1 :: CR1_SPEC > , # [doc = "0x04 - control register 2"] pub cr2 : crate :: Reg < cr2 :: CR2_SPEC > , _reserved2 : [u8 ; 0x04] , # [doc = "0x0c - DMA/Interrupt enable register"] pub dier : crate :: Reg < dier :: DIER_SPEC > , # [doc = "0x10 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x14 - event generation register"] pub egr : crate :: Reg < egr :: EGR_SPEC > , _reserved5 : [u8 ; 0x0c] , # [doc = "0x24 - counter"] pub cnt : crate :: Reg < cnt :: CNT_SPEC > , # [doc = "0x28 - prescaler"] pub psc : crate :: Reg < psc :: PSC_SPEC > , # [doc = "0x2c - auto-reload register"] pub arr : crate :: Reg < arr :: ARR_SPEC > , }
};
}

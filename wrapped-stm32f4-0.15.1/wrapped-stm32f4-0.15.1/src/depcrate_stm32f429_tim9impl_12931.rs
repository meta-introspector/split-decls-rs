// Generated macro for impl_12931 (impl)
macro_rules! Depcrate_stm32f429_tim9impl_12931 {
() => {
// Module: crate::stm32f429::tim9
// Provides: {"impl_12931"}
// Dependencies: {}
impl RegisterBlock { # [doc = "0x34 - capture/compare register"] # [inline (always)] pub fn ccr1 (& self) -> & crate :: Reg < ccr :: CCR_SPEC > { & self . ccr [0] } # [doc = "0x38 - capture/compare register"] # [inline (always)] pub fn ccr2 (& self) -> & crate :: Reg < ccr :: CCR_SPEC > { & self . ccr [1] } # [doc = "0x18 - capture/compare mode register 1 (input mode)"] # [inline (always)] pub fn ccmr1_input (& self) -> & crate :: Reg < ccmr1_input :: CCMR1_INPUT_SPEC > { unsafe { & * (((self as * const Self) as * const u8) . add (24usize) as * const crate :: Reg < ccmr1_input :: CCMR1_INPUT_SPEC >) } } # [doc = "0x18 - capture/compare mode register 1 (output mode)"] # [inline (always)] pub fn ccmr1_output (& self) -> & crate :: Reg < ccmr1_output :: CCMR1_OUTPUT_SPEC > { unsafe { & * (((self as * const Self) as * const u8) . add (24usize) as * const crate :: Reg < ccmr1_output :: CCMR1_OUTPUT_SPEC >) } } }
};
}

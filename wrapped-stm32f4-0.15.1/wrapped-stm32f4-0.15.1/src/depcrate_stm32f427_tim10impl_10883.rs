// Generated macro for impl_10883 (impl)
macro_rules! Depcrate_stm32f427_tim10impl_10883 {
() => {
// Module: crate::stm32f427::tim10
// Provides: {"impl_10883"}
// Dependencies: {}
impl RegisterBlock { # [doc = "0x34 - capture/compare register"] # [inline (always)] pub fn ccr1 (& self) -> & crate :: Reg < ccr :: CCR_SPEC > { & self . ccr [0] } # [doc = "0x18 - capture/compare mode register 1 (input mode)"] # [inline (always)] pub fn ccmr1_input (& self) -> & crate :: Reg < ccmr1_input :: CCMR1_INPUT_SPEC > { unsafe { & * (((self as * const Self) as * const u8) . add (24usize) as * const crate :: Reg < ccmr1_input :: CCMR1_INPUT_SPEC >) } } # [doc = "0x18 - capture/compare mode register 1 (output mode)"] # [inline (always)] pub fn ccmr1_output (& self) -> & crate :: Reg < ccmr1_output :: CCMR1_OUTPUT_SPEC > { unsafe { & * (((self as * const Self) as * const u8) . add (24usize) as * const crate :: Reg < ccmr1_output :: CCMR1_OUTPUT_SPEC >) } } }
};
}

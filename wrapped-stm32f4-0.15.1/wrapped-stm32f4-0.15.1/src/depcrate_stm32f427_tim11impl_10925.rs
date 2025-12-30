// Generated macro for impl_10925 (impl)
macro_rules! Depcrate_stm32f427_tim11impl_10925 {
() => {
// Module: crate::stm32f427::tim11
// Provides: {"impl_10925"}
// Dependencies: {}
impl RegisterBlock { # [doc = "0x34 - capture/compare register"] # [inline (always)] pub fn ccr1 (& self) -> & crate :: Reg < ccr :: CCR_SPEC > { & self . ccr [0] } # [doc = "0x18 - capture/compare mode register 1 (inputmode)"] # [inline (always)] pub fn ccmr1_input (& self) -> & crate :: Reg < ccmr1_input :: CCMR1_INPUT_SPEC > { unsafe { & * (((self as * const Self) as * const u8) . add (24usize) as * const crate :: Reg < ccmr1_input :: CCMR1_INPUT_SPEC >) } } # [doc = "0x18 - capture/compare mode register 1 (output mode)"] # [inline (always)] pub fn ccmr1_output (& self) -> & crate :: Reg < ccmr1_output :: CCMR1_OUTPUT_SPEC > { unsafe { & * (((self as * const Self) as * const u8) . add (24usize) as * const crate :: Reg < ccmr1_output :: CCMR1_OUTPUT_SPEC >) } } }
};
}

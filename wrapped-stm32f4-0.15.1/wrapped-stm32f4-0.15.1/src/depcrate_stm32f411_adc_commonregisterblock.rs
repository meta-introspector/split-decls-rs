// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f411_adc_commonRegisterBlock {
() => {
// Module: crate::stm32f411::adc_common
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { _reserved0 : [u8 ; 0x04] , # [doc = "0x04 - ADC common control register"] pub ccr : crate :: Reg < ccr :: CCR_SPEC > , }
};
}

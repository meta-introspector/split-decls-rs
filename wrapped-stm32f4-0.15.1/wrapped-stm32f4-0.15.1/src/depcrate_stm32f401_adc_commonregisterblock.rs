// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f401_adc_commonRegisterBlock {
() => {
// Module: crate::stm32f401::adc_common
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { _reserved0 : [u8 ; 0x04] , # [doc = "0x04 - ADC common control register"] pub ccr : crate :: Reg < ccr :: CCR_SPEC > , }
};
}

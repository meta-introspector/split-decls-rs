// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f410_adc_commonRegisterBlock {
() => {
// Module: crate::stm32f410::adc_common
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - ADC Common status register"] pub csr : crate :: Reg < csr :: CSR_SPEC > , # [doc = "0x04 - ADC common control register"] pub ccr : crate :: Reg < ccr :: CCR_SPEC > , }
};
}

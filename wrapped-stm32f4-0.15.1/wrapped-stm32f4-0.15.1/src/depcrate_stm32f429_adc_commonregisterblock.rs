// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f429_adc_commonRegisterBlock {
() => {
// Module: crate::stm32f429::adc_common
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - ADC Common status register"] pub csr : crate :: Reg < csr :: CSR_SPEC > , # [doc = "0x04 - ADC common control register"] pub ccr : crate :: Reg < ccr :: CCR_SPEC > , # [doc = "0x08 - ADC common regular data register for dual and triple modes"] pub cdr : crate :: Reg < cdr :: CDR_SPEC > , }
};
}

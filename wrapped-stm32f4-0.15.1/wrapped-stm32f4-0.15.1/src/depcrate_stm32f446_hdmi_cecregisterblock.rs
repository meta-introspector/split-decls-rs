// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f446_hdmi_cecRegisterBlock {
() => {
// Module: crate::stm32f446::hdmi_cec
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - CEC control register"] pub cec_cr : crate :: Reg < cec_cr :: CEC_CR_SPEC > , # [doc = "0x04 - CEC configuration register"] pub cec_cfgr : crate :: Reg < cec_cfgr :: CEC_CFGR_SPEC > , # [doc = "0x08 - CEC Tx data register"] pub cec_txdr : crate :: Reg < cec_txdr :: CEC_TXDR_SPEC > , # [doc = "0x0c - CEC Rx Data Register"] pub cec_rxdr : crate :: Reg < cec_rxdr :: CEC_RXDR_SPEC > , # [doc = "0x10 - CEC Interrupt and Status Register"] pub cec_isr : crate :: Reg < cec_isr :: CEC_ISR_SPEC > , # [doc = "0x14 - CEC interrupt enable register"] pub cec_ier : crate :: Reg < cec_ier :: CEC_IER_SPEC > , }
};
}

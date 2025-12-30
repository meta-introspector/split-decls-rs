// Generated macro for KEY (struct)
macro_rules! Depcrate_stm32f469_crypKEY {
() => {
// Module: crate::stm32f469::cryp
// Provides: {"KEY"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct KEY { # [doc = "0x00 - key registers"] pub klr : crate :: Reg < self :: key :: klr :: KLR_SPEC > , # [doc = "0x04 - key registers"] pub krr : crate :: Reg < self :: key :: krr :: KRR_SPEC > , }
};
}

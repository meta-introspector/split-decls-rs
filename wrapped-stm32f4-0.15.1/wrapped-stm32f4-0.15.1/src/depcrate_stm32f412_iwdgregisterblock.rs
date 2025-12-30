// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f412_iwdgRegisterBlock {
() => {
// Module: crate::stm32f412::iwdg
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Key register"] pub kr : crate :: Reg < kr :: KR_SPEC > , # [doc = "0x04 - Prescaler register"] pub pr : crate :: Reg < pr :: PR_SPEC > , # [doc = "0x08 - Reload register"] pub rlr : crate :: Reg < rlr :: RLR_SPEC > , # [doc = "0x0c - Status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , }
};
}

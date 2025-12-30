// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f446_sai1RegisterBlock {
() => {
// Module: crate::stm32f446::sai1
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Global configuration register"] pub gcr : crate :: Reg < gcr :: GCR_SPEC > , # [doc = "0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"] pub ch : [CH ; 2] , }
};
}

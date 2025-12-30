// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f413_saiRegisterBlock {
() => {
// Module: crate::stm32f413::sai
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { _reserved0 : [u8 ; 0x04] , # [doc = "0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"] pub ch : [CH ; 2] , }
};
}

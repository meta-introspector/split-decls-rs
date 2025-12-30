// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f405_sai1RegisterBlock {
() => {
// Module: crate::stm32f405::sai1
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { _reserved0 : [u8 ; 0x04] , # [doc = "0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"] pub ch : [CH ; 2] , }
};
}

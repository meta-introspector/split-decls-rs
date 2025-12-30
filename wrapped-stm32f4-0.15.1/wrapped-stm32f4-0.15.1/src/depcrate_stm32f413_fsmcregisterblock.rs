// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f413_fsmcRegisterBlock {
() => {
// Module: crate::stm32f413::fsmc
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - BCR1"] pub bcr1 : crate :: Reg < bcr1 :: BCR1_SPEC > , # [doc = "0x04 - BTR1"] pub btr1 : crate :: Reg < btr :: BTR_SPEC > , # [doc = "0x08 - BCR2"] pub bcr2 : crate :: Reg < bcr :: BCR_SPEC > , # [doc = "0x0c - BTR1"] pub btr2 : crate :: Reg < btr :: BTR_SPEC > , # [doc = "0x10 - BCR2"] pub bcr3 : crate :: Reg < bcr :: BCR_SPEC > , # [doc = "0x14 - BTR1"] pub btr3 : crate :: Reg < btr :: BTR_SPEC > , # [doc = "0x18 - BCR2"] pub bcr4 : crate :: Reg < bcr :: BCR_SPEC > , # [doc = "0x1c - BTR1"] pub btr4 : crate :: Reg < btr :: BTR_SPEC > , _reserved8 : [u8 ; 0xe4] , # [doc = "0x104 - BWTR1"] pub bwtr1 : crate :: Reg < bwtr :: BWTR_SPEC > , _reserved9 : [u8 ; 0x04] , # [doc = "0x10c - BWTR1"] pub bwtr2 : crate :: Reg < bwtr :: BWTR_SPEC > , _reserved10 : [u8 ; 0x04] , # [doc = "0x114 - BWTR1"] pub bwtr3 : crate :: Reg < bwtr :: BWTR_SPEC > , _reserved11 : [u8 ; 0x04] , # [doc = "0x11c - BWTR1"] pub bwtr4 : crate :: Reg < bwtr :: BWTR_SPEC > , }
};
}

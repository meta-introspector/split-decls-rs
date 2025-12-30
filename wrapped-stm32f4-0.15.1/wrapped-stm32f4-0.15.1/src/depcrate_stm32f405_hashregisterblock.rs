// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f405_hashRegisterBlock {
() => {
// Module: crate::stm32f405::hash
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - control register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x04 - data input register"] pub din : crate :: Reg < din :: DIN_SPEC > , # [doc = "0x08 - start register"] pub str : crate :: Reg < str :: STR_SPEC > , # [doc = "0x0c..0x20 - digest registers"] pub hr : [crate :: Reg < hr :: HR_SPEC > ; 5] , # [doc = "0x20 - interrupt enable register"] pub imr : crate :: Reg < imr :: IMR_SPEC > , # [doc = "0x24 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , _reserved6 : [u8 ; 0xd0] , # [doc = "0xf8..0x1d0 - context swap registers"] pub csr : [crate :: Reg < csr :: CSR_SPEC > ; 54] , _reserved7 : [u8 ; 0x0140] , # [doc = "0x310..0x330 - HASH digest register"] pub hash_hr : [crate :: Reg < hash_hr :: HASH_HR_SPEC > ; 8] , }
};
}

// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f413_aesRegisterBlock {
() => {
// Module: crate::stm32f413::aes
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - control register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x04 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x08 - data input register"] pub dinr : crate :: Reg < dinr :: DINR_SPEC > , # [doc = "0x0c - data output register"] pub doutr : crate :: Reg < doutr :: DOUTR_SPEC > , # [doc = "0x10 - key register 0"] pub keyr0 : crate :: Reg < keyr0 :: KEYR0_SPEC > , # [doc = "0x14 - key register 1"] pub keyr1 : crate :: Reg < keyr1 :: KEYR1_SPEC > , # [doc = "0x18 - key register 2"] pub keyr2 : crate :: Reg < keyr2 :: KEYR2_SPEC > , # [doc = "0x1c - key register 3"] pub keyr3 : crate :: Reg < keyr3 :: KEYR3_SPEC > , # [doc = "0x20 - initialization vector register 0"] pub ivr0 : crate :: Reg < ivr0 :: IVR0_SPEC > , # [doc = "0x24 - initialization vector register 1"] pub ivr1 : crate :: Reg < ivr1 :: IVR1_SPEC > , # [doc = "0x28 - initialization vector register 2"] pub ivr2 : crate :: Reg < ivr2 :: IVR2_SPEC > , # [doc = "0x2c - initialization vector register 3"] pub ivr3 : crate :: Reg < ivr3 :: IVR3_SPEC > , }
};
}

// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f405_flashRegisterBlock {
() => {
// Module: crate::stm32f405::flash
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Flash access control register"] pub acr : crate :: Reg < acr :: ACR_SPEC > , # [doc = "0x04 - Flash key register"] pub keyr : crate :: Reg < keyr :: KEYR_SPEC > , # [doc = "0x08 - Flash option key register"] pub optkeyr : crate :: Reg < optkeyr :: OPTKEYR_SPEC > , # [doc = "0x0c - Status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x10 - Control register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x14 - Flash option control register"] pub optcr : crate :: Reg < optcr :: OPTCR_SPEC > , }
};
}

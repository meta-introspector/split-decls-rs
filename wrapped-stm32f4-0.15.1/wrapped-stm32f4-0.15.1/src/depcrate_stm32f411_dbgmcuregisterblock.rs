// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f411_dbgmcuRegisterBlock {
() => {
// Module: crate::stm32f411::dbgmcu
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - IDCODE"] pub idcode : crate :: Reg < idcode :: IDCODE_SPEC > , # [doc = "0x04 - Control Register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x08 - Debug MCU APB1 Freeze registe"] pub apb1_fz : crate :: Reg < apb1_fz :: APB1_FZ_SPEC > , # [doc = "0x0c - Debug MCU APB2 Freeze registe"] pub apb2_fz : crate :: Reg < apb2_fz :: APB2_FZ_SPEC > , }
};
}

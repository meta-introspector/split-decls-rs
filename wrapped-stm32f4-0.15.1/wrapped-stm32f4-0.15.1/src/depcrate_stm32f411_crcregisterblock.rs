// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f411_crcRegisterBlock {
() => {
// Module: crate::stm32f411::crc
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Data register"] pub dr : crate :: Reg < dr :: DR_SPEC > , # [doc = "0x04 - Independent Data register"] pub idr : crate :: Reg < idr :: IDR_SPEC > , # [doc = "0x08 - Control register"] pub cr : crate :: Reg < cr :: CR_SPEC > , }
};
}

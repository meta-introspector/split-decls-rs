// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f427_crypRegisterBlock {
() => {
// Module: crate::stm32f427::cryp
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - control register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x04 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x08 - data input register"] pub din : crate :: Reg < din :: DIN_SPEC > , # [doc = "0x0c - data output register"] pub dout : crate :: Reg < dout :: DOUT_SPEC > , # [doc = "0x10 - DMA control register"] pub dmacr : crate :: Reg < dmacr :: DMACR_SPEC > , # [doc = "0x14 - interrupt mask set/clear register"] pub imscr : crate :: Reg < imscr :: IMSCR_SPEC > , # [doc = "0x18 - raw interrupt status register"] pub risr : crate :: Reg < risr :: RISR_SPEC > , # [doc = "0x1c - masked interrupt status register"] pub misr : crate :: Reg < misr :: MISR_SPEC > , # [doc = "0x20..0x40 - Cluster KEY%s, containing K?LR, K?RR"] pub key : [KEY ; 4] , # [doc = "0x40..0x50 - Cluster INIT%s, containing IV?LR, IV?RR"] pub init : [INIT ; 2] , # [doc = "0x50..0x70 - context swap register"] pub csgcmccmr : [crate :: Reg < csgcmccmr :: CSGCMCCMR_SPEC > ; 8] , # [doc = "0x70..0x90 - context swap register"] pub csgcmr : [crate :: Reg < csgcmr :: CSGCMR_SPEC > ; 8] , }
};
}

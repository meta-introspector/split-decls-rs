// Generated macro for ST (struct)
macro_rules! Depcrate_stm32f411_dma1ST {
() => {
// Module: crate::stm32f411::dma1
// Provides: {"ST"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct ST { # [doc = "0x00 - stream x configuration register"] pub cr : crate :: Reg < self :: st :: cr :: CR_SPEC > , # [doc = "0x04 - stream x number of data register"] pub ndtr : crate :: Reg < self :: st :: ndtr :: NDTR_SPEC > , # [doc = "0x08 - stream x peripheral address register"] pub par : crate :: Reg < self :: st :: par :: PAR_SPEC > , # [doc = "0x0c - stream x memory 0 address register"] pub m0ar : crate :: Reg < self :: st :: m0ar :: M0AR_SPEC > , # [doc = "0x10 - stream x memory 1 address register"] pub m1ar : crate :: Reg < self :: st :: m1ar :: M1AR_SPEC > , # [doc = "0x14 - stream x FIFO control register"] pub fcr : crate :: Reg < self :: st :: fcr :: FCR_SPEC > , }
};
}

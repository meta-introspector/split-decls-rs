// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f412_dma2RegisterBlock {
() => {
// Module: crate::stm32f412::dma2
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - low interrupt status register"] pub lisr : crate :: Reg < lisr :: LISR_SPEC > , # [doc = "0x04 - high interrupt status register"] pub hisr : crate :: Reg < hisr :: HISR_SPEC > , # [doc = "0x08 - low interrupt flag clear register"] pub lifcr : crate :: Reg < lifcr :: LIFCR_SPEC > , # [doc = "0x0c - high interrupt flag clear register"] pub hifcr : crate :: Reg < hifcr :: HIFCR_SPEC > , # [doc = "0x10..0xd0 - Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"] pub st : [ST ; 8] , }
};
}

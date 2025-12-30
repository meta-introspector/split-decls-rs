// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f469_dcmiRegisterBlock {
() => {
// Module: crate::stm32f469::dcmi
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - control register 1"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x04 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x08 - raw interrupt status register"] pub ris : crate :: Reg < ris :: RIS_SPEC > , # [doc = "0x0c - interrupt enable register"] pub ier : crate :: Reg < ier :: IER_SPEC > , # [doc = "0x10 - masked interrupt status register"] pub mis : crate :: Reg < mis :: MIS_SPEC > , # [doc = "0x14 - interrupt clear register"] pub icr : crate :: Reg < icr :: ICR_SPEC > , # [doc = "0x18 - embedded synchronization code register"] pub escr : crate :: Reg < escr :: ESCR_SPEC > , # [doc = "0x1c - embedded synchronization unmask register"] pub esur : crate :: Reg < esur :: ESUR_SPEC > , # [doc = "0x20 - crop window start"] pub cwstrt : crate :: Reg < cwstrt :: CWSTRT_SPEC > , # [doc = "0x24 - crop window size"] pub cwsize : crate :: Reg < cwsize :: CWSIZE_SPEC > , # [doc = "0x28 - data register"] pub dr : crate :: Reg < dr :: DR_SPEC > , }
};
}

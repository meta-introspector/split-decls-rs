// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f413_can1RegisterBlock {
() => {
// Module: crate::stm32f413::can1
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - master control register"] pub mcr : crate :: Reg < mcr :: MCR_SPEC > , # [doc = "0x04 - master status register"] pub msr : crate :: Reg < msr :: MSR_SPEC > , # [doc = "0x08 - transmit status register"] pub tsr : crate :: Reg < tsr :: TSR_SPEC > , # [doc = "0x0c..0x14 - receive FIFO %s register"] pub rfr : [crate :: Reg < rfr :: RFR_SPEC > ; 2] , # [doc = "0x14 - interrupt enable register"] pub ier : crate :: Reg < ier :: IER_SPEC > , # [doc = "0x18 - interrupt enable register"] pub esr : crate :: Reg < esr :: ESR_SPEC > , # [doc = "0x1c - bit timing register"] pub btr : crate :: Reg < btr :: BTR_SPEC > , _reserved7 : [u8 ; 0x0160] , # [doc = "0x180..0x1b0 - CAN Transmit cluster"] pub tx : [TX ; 3] , # [doc = "0x1b0..0x1d0 - CAN Receive cluster"] pub rx : [RX ; 2] , _reserved9 : [u8 ; 0x30] , # [doc = "0x200 - filter master register"] pub fmr : crate :: Reg < fmr :: FMR_SPEC > , # [doc = "0x204 - filter mode register"] pub fm1r : crate :: Reg < fm1r :: FM1R_SPEC > , _reserved11 : [u8 ; 0x04] , # [doc = "0x20c - filter scale register"] pub fs1r : crate :: Reg < fs1r :: FS1R_SPEC > , _reserved12 : [u8 ; 0x04] , # [doc = "0x214 - filter FIFO assignment register"] pub ffa1r : crate :: Reg < ffa1r :: FFA1R_SPEC > , _reserved13 : [u8 ; 0x04] , # [doc = "0x21c - filter activation register"] pub fa1r : crate :: Reg < fa1r :: FA1R_SPEC > , _reserved14 : [u8 ; 0x20] , # [doc = "0x240..0x320 - CAN Filter Bank cluster"] pub fb : [FB ; 28] , }
};
}

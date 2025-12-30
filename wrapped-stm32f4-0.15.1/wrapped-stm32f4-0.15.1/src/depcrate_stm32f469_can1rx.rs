// Generated macro for RX (struct)
macro_rules! Depcrate_stm32f469_can1RX {
() => {
// Module: crate::stm32f469::can1
// Provides: {"RX"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RX { # [doc = "0x00 - receive FIFO mailbox identifier register"] pub rir : crate :: Reg < self :: rx :: rir :: RIR_SPEC > , # [doc = "0x04 - mailbox data high register"] pub rdtr : crate :: Reg < self :: rx :: rdtr :: RDTR_SPEC > , # [doc = "0x08 - mailbox data high register"] pub rdlr : crate :: Reg < self :: rx :: rdlr :: RDLR_SPEC > , # [doc = "0x0c - receive FIFO mailbox data high register"] pub rdhr : crate :: Reg < self :: rx :: rdhr :: RDHR_SPEC > , }
};
}

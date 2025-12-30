// Generated macro for TX (struct)
macro_rules! Depcrate_stm32f469_can1TX {
() => {
// Module: crate::stm32f469::can1
// Provides: {"TX"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct TX { # [doc = "0x00 - TX mailbox identifier register"] pub tir : crate :: Reg < self :: tx :: tir :: TIR_SPEC > , # [doc = "0x04 - mailbox data length control and time stamp register"] pub tdtr : crate :: Reg < self :: tx :: tdtr :: TDTR_SPEC > , # [doc = "0x08 - mailbox data low register"] pub tdlr : crate :: Reg < self :: tx :: tdlr :: TDLR_SPEC > , # [doc = "0x0c - mailbox data high register"] pub tdhr : crate :: Reg < self :: tx :: tdhr :: TDHR_SPEC > , }
};
}

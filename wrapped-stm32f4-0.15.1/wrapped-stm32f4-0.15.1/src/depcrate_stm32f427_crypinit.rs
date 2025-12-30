// Generated macro for INIT (struct)
macro_rules! Depcrate_stm32f427_crypINIT {
() => {
// Module: crate::stm32f427::cryp
// Provides: {"INIT"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct INIT { # [doc = "0x00 - initialization vector registers"] pub ivlr : crate :: Reg < self :: init :: ivlr :: IVLR_SPEC > , # [doc = "0x04 - initialization vector registers"] pub ivrr : crate :: Reg < self :: init :: ivrr :: IVRR_SPEC > , }
};
}

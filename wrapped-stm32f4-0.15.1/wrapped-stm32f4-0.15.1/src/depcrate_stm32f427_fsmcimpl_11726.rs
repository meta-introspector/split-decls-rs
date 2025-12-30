// Generated macro for impl_11726 (impl)
macro_rules! Depcrate_stm32f427_fsmcimpl_11726 {
() => {
// Module: crate::stm32f427::fsmc
// Provides: {"impl_11726"}
// Dependencies: {}
impl RegisterBlock { # [doc = "0x140 - SDRAM Control Register 1"] # [inline (always)] pub fn sdcr1 (& self) -> & crate :: Reg < sdcr :: SDCR_SPEC > { & self . sdcr [0] } # [doc = "0x144 - SDRAM Control Register 1"] # [inline (always)] pub fn sdcr2 (& self) -> & crate :: Reg < sdcr :: SDCR_SPEC > { & self . sdcr [1] } # [doc = "0x148 - SDRAM Timing register 1"] # [inline (always)] pub fn sdtr1 (& self) -> & crate :: Reg < sdtr :: SDTR_SPEC > { & self . sdtr [0] } # [doc = "0x14c - SDRAM Timing register 1"] # [inline (always)] pub fn sdtr2 (& self) -> & crate :: Reg < sdtr :: SDTR_SPEC > { & self . sdtr [1] } }
};
}

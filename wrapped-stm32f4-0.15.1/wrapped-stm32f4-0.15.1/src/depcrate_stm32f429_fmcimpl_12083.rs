// Generated macro for impl_12083 (impl)
macro_rules! Depcrate_stm32f429_fmcimpl_12083 {
() => {
// Module: crate::stm32f429::fmc
// Provides: {"impl_12083"}
// Dependencies: {}
impl RegisterBlock { # [doc = "0x140 - SDRAM Control Register 1"] # [inline (always)] pub fn sdcr1 (& self) -> & crate :: Reg < sdcr :: SDCR_SPEC > { & self . sdcr [0] } # [doc = "0x144 - SDRAM Control Register 1"] # [inline (always)] pub fn sdcr2 (& self) -> & crate :: Reg < sdcr :: SDCR_SPEC > { & self . sdcr [1] } # [doc = "0x148 - SDRAM Timing register 1"] # [inline (always)] pub fn sdtr1 (& self) -> & crate :: Reg < sdtr :: SDTR_SPEC > { & self . sdtr [0] } # [doc = "0x14c - SDRAM Timing register 1"] # [inline (always)] pub fn sdtr2 (& self) -> & crate :: Reg < sdtr :: SDTR_SPEC > { & self . sdtr [1] } }
};
}

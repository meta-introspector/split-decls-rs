// Generated macro for impl_1819 (impl)
macro_rules! Depcrate_stm32f405_rtcimpl_1819 {
() => {
// Module: crate::stm32f405::rtc
// Provides: {"impl_1819"}
// Dependencies: {}
impl RegisterBlock { # [doc = "0x1c - Alarm register"] # [inline (always)] pub fn alrmar (& self) -> & crate :: Reg < alrmr :: ALRMR_SPEC > { & self . alrmr [0] } # [doc = "0x20 - Alarm register"] # [inline (always)] pub fn alrmbr (& self) -> & crate :: Reg < alrmr :: ALRMR_SPEC > { & self . alrmr [1] } # [doc = "0x44 - Alarm sub-second register"] # [inline (always)] pub fn alrmassr (& self) -> & crate :: Reg < alrmssr :: ALRMSSR_SPEC > { & self . alrmssr [0] } # [doc = "0x48 - Alarm sub-second register"] # [inline (always)] pub fn alrmbssr (& self) -> & crate :: Reg < alrmssr :: ALRMSSR_SPEC > { & self . alrmssr [1] } }
};
}

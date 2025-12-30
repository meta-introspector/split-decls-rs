// Generated macro for impl_12659 (impl)
macro_rules! Depcrate_stm32f429_rtcimpl_12659 {
() => {
// Module: crate::stm32f429::rtc
// Provides: {"impl_12659"}
// Dependencies: {}
impl RegisterBlock { # [doc = "0x1c - Alarm register"] # [inline (always)] pub fn alrmar (& self) -> & crate :: Reg < alrmr :: ALRMR_SPEC > { & self . alrmr [0] } # [doc = "0x20 - Alarm register"] # [inline (always)] pub fn alrmbr (& self) -> & crate :: Reg < alrmr :: ALRMR_SPEC > { & self . alrmr [1] } # [doc = "0x44 - Alarm sub-second register"] # [inline (always)] pub fn alrmassr (& self) -> & crate :: Reg < alrmssr :: ALRMSSR_SPEC > { & self . alrmssr [0] } # [doc = "0x48 - Alarm sub-second register"] # [inline (always)] pub fn alrmbssr (& self) -> & crate :: Reg < alrmssr :: ALRMSSR_SPEC > { & self . alrmssr [1] } }
};
}

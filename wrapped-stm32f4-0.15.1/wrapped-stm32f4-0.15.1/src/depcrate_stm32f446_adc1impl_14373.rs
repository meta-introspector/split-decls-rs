// Generated macro for impl_14373 (impl)
macro_rules! Depcrate_stm32f446_adc1impl_14373 {
() => {
// Module: crate::stm32f446::adc1
// Provides: {"impl_14373"}
// Dependencies: {}
impl RegisterBlock { # [doc = "0x14 - injected channel data offset register x"] # [inline (always)] pub fn jofr1 (& self) -> & crate :: Reg < jofr :: JOFR_SPEC > { & self . jofr [0] } # [doc = "0x18 - injected channel data offset register x"] # [inline (always)] pub fn jofr2 (& self) -> & crate :: Reg < jofr :: JOFR_SPEC > { & self . jofr [1] } # [doc = "0x1c - injected channel data offset register x"] # [inline (always)] pub fn jofr3 (& self) -> & crate :: Reg < jofr :: JOFR_SPEC > { & self . jofr [2] } # [doc = "0x20 - injected channel data offset register x"] # [inline (always)] pub fn jofr4 (& self) -> & crate :: Reg < jofr :: JOFR_SPEC > { & self . jofr [3] } # [doc = "0x3c - injected data register x"] # [inline (always)] pub fn jdr1 (& self) -> & crate :: Reg < jdr :: JDR_SPEC > { & self . jdr [0] } # [doc = "0x40 - injected data register x"] # [inline (always)] pub fn jdr2 (& self) -> & crate :: Reg < jdr :: JDR_SPEC > { & self . jdr [1] } # [doc = "0x44 - injected data register x"] # [inline (always)] pub fn jdr3 (& self) -> & crate :: Reg < jdr :: JDR_SPEC > { & self . jdr [2] } # [doc = "0x48 - injected data register x"] # [inline (always)] pub fn jdr4 (& self) -> & crate :: Reg < jdr :: JDR_SPEC > { & self . jdr [3] } }
};
}

// Generated macro for impl_9802 (impl)
macro_rules! Depcrate_stm32f413_otg_fs_globalimpl_9802 {
() => {
// Module: crate::stm32f413::otg_fs_global
// Provides: {"impl_9802"}
// Dependencies: {}
impl RegisterBlock { # [doc = "0x1c - OTG_FS Receive status debug read(Host mode)"] # [inline (always)] pub fn grxstsr_host (& self) -> & crate :: Reg < grxstsr_host :: GRXSTSR_HOST_SPEC > { unsafe { & * (((self as * const Self) as * const u8) . add (28usize) as * const crate :: Reg < grxstsr_host :: GRXSTSR_HOST_SPEC >) } } # [doc = "0x1c - OTG_FS Receive status debug read(Device mode)"] # [inline (always)] pub fn grxstsr_device (& self) -> & crate :: Reg < grxstsr_device :: GRXSTSR_DEVICE_SPEC > { unsafe { & * (((self as * const Self) as * const u8) . add (28usize) as * const crate :: Reg < grxstsr_device :: GRXSTSR_DEVICE_SPEC >) } } # [doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)"] # [inline (always)] pub fn gnptxfsiz_host (& self) -> & crate :: Reg < gnptxfsiz_host :: GNPTXFSIZ_HOST_SPEC > { unsafe { & * (((self as * const Self) as * const u8) . add (40usize) as * const crate :: Reg < gnptxfsiz_host :: GNPTXFSIZ_HOST_SPEC >) } } # [doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)"] # [inline (always)] pub fn gnptxfsiz_device (& self) -> & crate :: Reg < gnptxfsiz_device :: GNPTXFSIZ_DEVICE_SPEC > { unsafe { & * (((self as * const Self) as * const u8) . add (40usize) as * const crate :: Reg < gnptxfsiz_device :: GNPTXFSIZ_DEVICE_SPEC >) } } }
};
}

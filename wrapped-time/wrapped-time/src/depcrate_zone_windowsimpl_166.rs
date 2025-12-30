// Generated macro for impl_166 (impl)
macro_rules! Depcrate_zone_windowsimpl_166 {
() => {
// Module: crate::zone::windows
// Provides: {"impl_166"}
// Dependencies: {}
impl WindowsParserBorrowed < 'static > { # [doc = " Cheaply converts a [`WindowsParserBorrowed<'static>`] into a [`WindowsParser`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`WindowsParser`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`WindowsParserBorrowed`]."] pub fn static_to_owned (& self) -> WindowsParser { WindowsParser { data : DataPayload :: from_static_ref (self . data) , } } }
};
}

// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl < W : WriteColor > WriteColor for LossyStandardStream < W > { fn supports_color (& self) -> bool { self . wtr . supports_color () } fn supports_hyperlinks (& self) -> bool { self . wtr . supports_hyperlinks () } fn set_color (& mut self , spec : & ColorSpec) -> io :: Result < () > { self . wtr . set_color (spec) } fn set_hyperlink (& mut self , link : & HyperlinkSpec) -> io :: Result < () > { self . wtr . set_hyperlink (link) } fn reset (& mut self) -> io :: Result < () > { self . wtr . reset () } fn is_synchronous (& self) -> bool { self . wtr . is_synchronous () } }
};
}

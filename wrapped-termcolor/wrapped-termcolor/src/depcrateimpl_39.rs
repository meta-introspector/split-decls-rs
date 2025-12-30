// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl WriteColor for BufferedStandardStream { # [inline] fn supports_color (& self) -> bool { self . wtr . supports_color () } # [inline] fn supports_hyperlinks (& self) -> bool { self . wtr . supports_hyperlinks () } # [inline] fn set_color (& mut self , spec : & ColorSpec) -> io :: Result < () > { if self . is_synchronous () { self . wtr . flush () ? ; } self . wtr . set_color (spec) } # [inline] fn set_hyperlink (& mut self , link : & HyperlinkSpec) -> io :: Result < () > { if self . is_synchronous () { self . wtr . flush () ? ; } self . wtr . set_hyperlink (link) } # [inline] fn reset (& mut self) -> io :: Result < () > { self . wtr . reset () } # [inline] fn is_synchronous (& self) -> bool { self . wtr . is_synchronous () } }
};
}

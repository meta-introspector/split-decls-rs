// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl WriteColor for StandardStream { # [inline] fn supports_color (& self) -> bool { self . wtr . supports_color () } # [inline] fn supports_hyperlinks (& self) -> bool { self . wtr . supports_hyperlinks () } # [inline] fn set_color (& mut self , spec : & ColorSpec) -> io :: Result < () > { self . wtr . set_color (spec) } # [inline] fn set_hyperlink (& mut self , link : & HyperlinkSpec) -> io :: Result < () > { self . wtr . set_hyperlink (link) } # [inline] fn reset (& mut self) -> io :: Result < () > { self . wtr . reset () } # [inline] fn is_synchronous (& self) -> bool { self . wtr . is_synchronous () } }
};
}

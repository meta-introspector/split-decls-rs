// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < T : ? Sized + WriteColor > WriteColor for Box < T > { fn supports_color (& self) -> bool { (& * * self) . supports_color () } fn supports_hyperlinks (& self) -> bool { (& * * self) . supports_hyperlinks () } fn set_color (& mut self , spec : & ColorSpec) -> io :: Result < () > { (& mut * * self) . set_color (spec) } fn set_hyperlink (& mut self , link : & HyperlinkSpec) -> io :: Result < () > { (& mut * * self) . set_hyperlink (link) } fn reset (& mut self) -> io :: Result < () > { (& mut * * self) . reset () } fn is_synchronous (& self) -> bool { (& * * self) . is_synchronous () } }
};
}

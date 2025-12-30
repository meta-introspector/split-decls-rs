// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a , T : ? Sized + WriteColor > WriteColor for & 'a mut T { fn supports_color (& self) -> bool { (& * * self) . supports_color () } fn supports_hyperlinks (& self) -> bool { (& * * self) . supports_hyperlinks () } fn set_color (& mut self , spec : & ColorSpec) -> io :: Result < () > { (& mut * * self) . set_color (spec) } fn set_hyperlink (& mut self , link : & HyperlinkSpec) -> io :: Result < () > { (& mut * * self) . set_hyperlink (link) } fn reset (& mut self) -> io :: Result < () > { (& mut * * self) . reset () } fn is_synchronous (& self) -> bool { (& * * self) . is_synchronous () } }
};
}

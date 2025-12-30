// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl < W : io :: Write > WriteColor for NoColor < W > { # [inline] fn supports_color (& self) -> bool { false } # [inline] fn supports_hyperlinks (& self) -> bool { false } # [inline] fn set_color (& mut self , _ : & ColorSpec) -> io :: Result < () > { Ok (()) } # [inline] fn set_hyperlink (& mut self , _ : & HyperlinkSpec) -> io :: Result < () > { Ok (()) } # [inline] fn reset (& mut self) -> io :: Result < () > { Ok (()) } # [inline] fn is_synchronous (& self) -> bool { false } }
};
}

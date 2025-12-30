// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (windows)] impl WriteColor for WindowsBuffer { # [inline] fn supports_color (& self) -> bool { true } # [inline] fn supports_hyperlinks (& self) -> bool { false } # [inline] fn set_color (& mut self , spec : & ColorSpec) -> io :: Result < () > { self . push (Some (spec . clone ())) ; Ok (()) } # [inline] fn set_hyperlink (& mut self , _ : & HyperlinkSpec) -> io :: Result < () > { Ok (()) } # [inline] fn reset (& mut self) -> io :: Result < () > { self . push (None) ; Ok (()) } # [inline] fn is_synchronous (& self) -> bool { false } }
};
}

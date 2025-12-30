// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl < W : io :: Write > WriteColor for Ansi < W > { # [inline] fn supports_color (& self) -> bool { true } # [inline] fn supports_hyperlinks (& self) -> bool { true } # [inline] fn set_color (& mut self , spec : & ColorSpec) -> io :: Result < () > { if spec . reset { self . reset () ? ; } if spec . bold { self . write_str ("\x1B[1m") ? ; } if spec . dimmed { self . write_str ("\x1B[2m") ? ; } if spec . italic { self . write_str ("\x1B[3m") ? ; } if spec . underline { self . write_str ("\x1B[4m") ? ; } if spec . strikethrough { self . write_str ("\x1B[9m") ? ; } if let Some (ref c) = spec . fg_color { self . write_color (true , c , spec . intense) ? ; } if let Some (ref c) = spec . bg_color { self . write_color (false , c , spec . intense) ? ; } Ok (()) } # [inline] fn set_hyperlink (& mut self , link : & HyperlinkSpec) -> io :: Result < () > { self . write_str ("\x1B]8;;") ? ; if let Some (uri) = link . uri () { self . write_all (uri) ? ; } self . write_str ("\x1B\\") } # [inline] fn reset (& mut self) -> io :: Result < () > { self . write_str ("\x1B[0m") } # [inline] fn is_synchronous (& self) -> bool { false } }
};
}

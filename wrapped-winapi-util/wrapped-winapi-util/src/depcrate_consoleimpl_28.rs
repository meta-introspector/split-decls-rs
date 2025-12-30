// Generated macro for impl_28 (impl)
macro_rules! Depcrate_consoleimpl_28 {
() => {
// Module: crate::console
// Provides: {"impl_28"}
// Dependencies: {}
impl TextAttributes { fn to_word (& self) -> u16 { let mut w = 0 ; w |= self . fg_color . to_fg () ; w |= self . fg_intense . to_fg () ; w |= self . bg_color . to_bg () ; w |= self . bg_intense . to_bg () ; w } fn from_word (word : u16) -> TextAttributes { TextAttributes { fg_color : Color :: from_fg (word) , fg_intense : Intense :: from_fg (word) , bg_color : Color :: from_bg (word) , bg_intense : Intense :: from_bg (word) , } } }
};
}

// Generated macro for impl_18 (impl)
macro_rules! Depcrate_winimpl_18 {
() => {
// Module: crate::win
// Provides: {"impl_18"}
// Dependencies: {}
impl TextAttributes { fn to_word (& self) -> WORD { let mut w = 0 ; w |= self . fg_color . to_fg () ; w |= self . fg_intense . to_fg () ; w |= self . bg_color . to_bg () ; w |= self . bg_intense . to_bg () ; w } fn from_word (word : WORD) -> TextAttributes { TextAttributes { fg_color : Color :: from_fg (word) , fg_intense : Intense :: from_fg (word) , bg_color : Color :: from_bg (word) , bg_intense : Intense :: from_bg (word) , } } }
};
}

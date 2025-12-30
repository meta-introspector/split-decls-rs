// Generated macro for impl_22 (impl)
macro_rules! Depcrate_winimpl_22 {
() => {
// Module: crate::win
// Provides: {"impl_22"}
// Dependencies: {}
impl Color { fn to_bg (& self) -> WORD { self . to_fg () << 4 } fn from_bg (word : WORD) -> Color { Color :: from_fg (word >> 4) } fn to_fg (& self) -> WORD { match * self { Color :: Black => 0 , Color :: Blue => FG_BLUE , Color :: Green => FG_GREEN , Color :: Red => FG_RED , Color :: Cyan => FG_CYAN , Color :: Magenta => FG_MAGENTA , Color :: Yellow => FG_YELLOW , Color :: White => FG_WHITE , } } fn from_fg (word : WORD) -> Color { match word & 0b111 { FG_BLUE => Color :: Blue , FG_GREEN => Color :: Green , FG_RED => Color :: Red , FG_CYAN => Color :: Cyan , FG_MAGENTA => Color :: Magenta , FG_YELLOW => Color :: Yellow , FG_WHITE => Color :: White , _ => Color :: Black , } } }
};
}

// Generated macro for impl_20 (impl)
macro_rules! Depcrate_winimpl_20 {
() => {
// Module: crate::win
// Provides: {"impl_20"}
// Dependencies: {}
impl Intense { fn to_bg (& self) -> WORD { self . to_fg () << 4 } fn from_bg (word : WORD) -> Intense { Intense :: from_fg (word >> 4) } fn to_fg (& self) -> WORD { match * self { Intense :: No => 0 , Intense :: Yes => FG_INTENSITY , } } fn from_fg (word : WORD) -> Intense { if word & FG_INTENSITY > 0 { Intense :: Yes } else { Intense :: No } } }
};
}

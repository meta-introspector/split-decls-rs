// Generated macro for impl_30 (impl)
macro_rules! Depcrate_consoleimpl_30 {
() => {
// Module: crate::console
// Provides: {"impl_30"}
// Dependencies: {}
impl Intense { fn to_bg (& self) -> u16 { self . to_fg () << 4 } fn from_bg (word : u16) -> Intense { Intense :: from_fg (word >> 4) } fn to_fg (& self) -> u16 { match * self { Intense :: No => 0 , Intense :: Yes => FG_INTENSITY , } } fn from_fg (word : u16) -> Intense { if word & FG_INTENSITY > 0 { Intense :: Yes } else { Intense :: No } } }
};
}

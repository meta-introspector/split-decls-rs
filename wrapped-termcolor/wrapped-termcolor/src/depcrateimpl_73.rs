// Generated macro for impl_73 (impl)
macro_rules! Depcrateimpl_73 {
() => {
// Module: crate
// Provides: {"impl_73"}
// Dependencies: {}
impl error :: Error for ParseColorError { fn description (& self) -> & str { use self :: ParseColorErrorKind :: * ; match self . kind { InvalidName => "unrecognized color name" , InvalidAnsi256 => "invalid ansi256 color number" , InvalidRgb => "invalid RGB color triple" , } } }
};
}

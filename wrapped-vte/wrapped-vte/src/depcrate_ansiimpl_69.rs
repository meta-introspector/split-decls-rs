// Generated macro for impl_69 (impl)
macro_rules! Depcrate_ansiimpl_69 {
() => {
// Module: crate::ansi
// Provides: {"impl_69"}
// Dependencies: {}
impl Mode { fn new (mode : u16) -> Self { match mode { 4 => Self :: Named (NamedMode :: Insert) , 20 => Self :: Named (NamedMode :: LineFeedNewLine) , _ => Self :: Unknown (mode) , } } # [doc = " Get the raw value of the mode."] pub fn raw (self) -> u16 { match self { Self :: Named (named) => named as u16 , Self :: Unknown (mode) => mode , } } }
};
}

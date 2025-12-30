// Generated macro for impl_310 (impl)
macro_rules! Depcrate_flagimpl_310 {
() => {
// Module: crate::flag
// Provides: {"impl_310"}
// Dependencies: {}
impl ProcSchemeVerb { pub fn try_from_raw (verb : u8) -> Option < Self > { Some (match verb { 255 => Self :: Iopl , _ => return None , }) } }
};
}

// Generated macro for impl_2778 (impl)
macro_rules! Depcrate_panicimpl_2778 {
() => {
// Module: crate::panic
// Provides: {"impl_2778"}
// Dependencies: {}
impl BacktraceStyle { pub (crate) fn full () -> Option < Self > { if cfg ! (feature = "backtrace") { Some (BacktraceStyle :: Full) } else { None } } fn as_u8 (self) -> u8 { match self { BacktraceStyle :: Short => 1 , BacktraceStyle :: Full => 2 , BacktraceStyle :: Off => 3 , } } fn from_u8 (s : u8) -> Option < Self > { match s { 1 => Some (BacktraceStyle :: Short) , 2 => Some (BacktraceStyle :: Full) , 3 => Some (BacktraceStyle :: Off) , _ => None , } } }
};
}

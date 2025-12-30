// Generated macro for impl_20 (impl)
macro_rules! Depcrate_printer_prettyimpl_20 {
() => {
// Module: crate::printer::pretty
// Provides: {"impl_20"}
// Dependencies: {}
impl Indent { fn repr (& self) -> & 'static str { match self { Self :: Null => "   " , Self :: Line => "│  " , Self :: Fork => "┝━ " , Self :: Turn => "┕━ " , } } }
};
}

// Generated macro for payload_as_str (function)
macro_rules! Depcrate_panickingpayload_as_str {
() => {
// Module: crate::panicking
// Provides: {"payload_as_str"}
// Dependencies: {}
fn payload_as_str (payload : & dyn Any) -> & str { if let Some (& s) = payload . downcast_ref :: < & 'static str > () { s } else if let Some (s) = payload . downcast_ref :: < String > () { s . as_str () } else { "Box<dyn Any>" } }
};
}

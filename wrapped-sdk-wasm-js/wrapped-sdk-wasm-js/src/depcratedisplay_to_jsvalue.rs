// Generated macro for display_to_jsvalue (function)
macro_rules! Depcratedisplay_to_jsvalue {
() => {
// Module: crate
// Provides: {"display_to_jsvalue"}
// Dependencies: {}
pub fn display_to_jsvalue < T : std :: fmt :: Display > (display : T) -> JsValue { display . to_string () . into () }
};
}

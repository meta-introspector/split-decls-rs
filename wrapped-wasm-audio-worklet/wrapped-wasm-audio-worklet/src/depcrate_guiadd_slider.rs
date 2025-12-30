// Generated macro for add_slider (function)
macro_rules! Depcrate_guiadd_slider {
() => {
// Module: crate::gui
// Provides: {"add_slider"}
// Dependencies: {}
fn add_slider (document : & web_sys :: Document , body : & web_sys :: HtmlElement , name : & str ,) -> Result < HtmlInputElement , JsValue > { let input : HtmlInputElement = document . create_element ("input") ? . unchecked_into () ; let label : HtmlLabelElement = document . create_element ("label") ? . unchecked_into () ; input . set_type ("range") ; label . set_text_content (Some (name)) ; label . append_child (& input) ? ; body . append_child (& label) ? ; Ok (input) }
};
}

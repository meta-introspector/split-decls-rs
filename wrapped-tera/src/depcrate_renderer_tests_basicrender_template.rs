// Generated macro for render_template (function)
macro_rules! Depcrate_renderer_tests_basicrender_template {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"render_template"}
// Dependencies: {}
fn render_template (content : & str , context : & Context) -> Result < String > { let mut tera = Tera :: default () ; tera . add_raw_template ("hello.html" , content) . unwrap () ; tera . register_function ("get_number" , | _ : & HashMap < String , Value > | Ok (Value :: Number (10 . into ()))) ; tera . register_function ("get_true" , | _ : & HashMap < String , Value > | Ok (Value :: Bool (true))) ; tera . register_function ("get_string" , | _ : & HashMap < String , Value > | { Ok (Value :: String ("Hello" . to_string ())) }) ; tera . render ("hello.html" , context) }
};
}

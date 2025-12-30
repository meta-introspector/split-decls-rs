// Generated macro for setup_input_oninput_callback (function)
macro_rules! Depcratesetup_input_oninput_callback {
() => {
// Module: crate
// Provides: {"setup_input_oninput_callback"}
// Dependencies: {}
fn setup_input_oninput_callback (worker : Rc < RefCell < web_sys :: Worker > >) { let document = web_sys :: window () . unwrap () . document () . unwrap () ; # [allow (unused_assignments)] let mut persistent_callback_handle = get_on_msg_callback () ; let callback = Closure :: new (move | | { console :: log_1 (& "oninput callback triggered" . into ()) ; let document = web_sys :: window () . unwrap () . document () . unwrap () ; let input_field = document . get_element_by_id ("inputNumber") . expect ("#inputNumber should exist") ; let input_field = input_field . dyn_ref :: < HtmlInputElement > () . expect ("#inputNumber should be a HtmlInputElement") ; match input_field . value () . parse :: < i32 > () { Ok (number) => { let worker_handle = & * worker . borrow () ; let _ = worker_handle . post_message (& number . into ()) ; persistent_callback_handle = get_on_msg_callback () ; worker_handle . set_onmessage (Some (persistent_callback_handle . as_ref () . unchecked_ref ())) ; } Err (_) => { document . get_element_by_id ("resultField") . expect ("#resultField should exist") . dyn_ref :: < HtmlElement > () . expect ("#resultField should be a HtmlInputElement") . set_inner_text ("") ; } } }) ; document . get_element_by_id ("inputNumber") . expect ("#inputNumber should exist") . dyn_ref :: < HtmlInputElement > () . expect ("#inputNumber should be a HtmlInputElement") . set_oninput (Some (callback . as_ref () . unchecked_ref ())) ; callback . forget () ; }
};
}

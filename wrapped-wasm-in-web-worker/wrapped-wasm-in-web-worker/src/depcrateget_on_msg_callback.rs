// Generated macro for get_on_msg_callback (function)
macro_rules! Depcrateget_on_msg_callback {
() => {
// Module: crate
// Provides: {"get_on_msg_callback"}
// Dependencies: {}
# [doc = " Create a closure to act on the message returned by the worker"] fn get_on_msg_callback () -> Closure < dyn FnMut (MessageEvent) > { Closure :: new (move | event : MessageEvent | { console :: log_2 (& "Received response: " . into () , & event . data ()) ; let result = match event . data () . as_bool () . unwrap () { true => "even" , false => "odd" , } ; let document = web_sys :: window () . unwrap () . document () . unwrap () ; document . get_element_by_id ("resultField") . expect ("#resultField should exist") . dyn_ref :: < HtmlElement > () . expect ("#resultField should be a HtmlInputElement") . set_inner_text (result) ; }) }
};
}

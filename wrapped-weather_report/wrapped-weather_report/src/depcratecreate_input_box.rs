// Generated macro for create_input_box (function)
macro_rules! Depcratecreate_input_box {
() => {
// Module: crate
// Provides: {"create_input_box"}
// Dependencies: {}
fn create_input_box (document : & Document) -> Element { let input_box = document . create_element ("input") . unwrap () ; input_box . set_attribute ("name" , "name") . expect ("failed to set attr name to name") ; input_box . set_attribute ("value" , "Delhi") . expect ("
    failed to set attr value to Delhi" ,) ; input_box . set_attribute ("type" , "text") . expect ("failed to set attr type to text") ; input_box . set_attribute ("placeholder" , "Type city name here") . expect ("Failed to set attr placeholder to Type city name here") ; input_box . set_id ("name") ; input_box . set_class_name ("ReportStyles-search") ; input_box }
};
}

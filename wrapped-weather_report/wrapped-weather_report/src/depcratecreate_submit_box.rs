// Generated macro for create_submit_box (function)
macro_rules! Depcratecreate_submit_box {
() => {
// Module: crate
// Provides: {"create_submit_box"}
// Dependencies: {}
fn create_submit_box (document : & Document) -> Element { let submit_box : Element = document . create_element ("input") . unwrap () ; submit_box . set_attribute ("type" , "button") . expect ("failed to set attr type to button") ; submit_box . set_attribute ("value" , "Search") . expect ("failed to set attr value to Search") ; submit_box . set_attribute ("name" , "submit") . expect ("failed to set attr name to submit") ; submit_box . set_id ("submit") ; submit_box . set_class_name (" ReportStyles-bootstrapButton btn btn-info") ; submit_box }
};
}

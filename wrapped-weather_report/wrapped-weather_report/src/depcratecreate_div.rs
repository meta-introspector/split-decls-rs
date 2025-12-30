// Generated macro for create_div (function)
macro_rules! Depcratecreate_div {
() => {
// Module: crate
// Provides: {"create_div"}
// Dependencies: {}
fn create_div (document : & Document , id : & str , class : & str) -> Element { let div = document . create_element ("div") . unwrap () ; div . set_id (id) ; div . set_class_name (class) ; div }
};
}

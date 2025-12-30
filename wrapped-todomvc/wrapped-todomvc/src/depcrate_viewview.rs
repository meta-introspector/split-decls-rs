// Generated macro for View (struct)
macro_rules! Depcrate_viewView {
() => {
// Module: crate::view
// Provides: {"View"}
// Dependencies: {}
# [doc = " Presentation layer"] # [wasm_bindgen] pub struct View { sched : RefCell < Rc < Scheduler > > , todo_list : Element , todo_item_counter : Element , clear_completed : Element , main : Element , toggle_all : Element , new_todo : Element , callbacks : Vec < (web_sys :: EventTarget , String , Closure < dyn FnMut () >) > , }
};
}

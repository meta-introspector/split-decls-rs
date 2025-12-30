// Generated macro for impl_62 (impl)
macro_rules! Depcrate_templateimpl_62 {
() => {
// Module: crate::template
// Provides: {"impl_62"}
// Dependencies: {}
impl Template { # [doc = " Format the contents of a todo list."] # [doc = ""] # [doc = " items `ItemList` contains keys you want to find in the template to replace."] # [doc = " Returns the contents for a todo list"] # [doc = ""] pub fn item_list (items : ItemList) -> String { let mut output = String :: from ("") ; for item in items . iter () { let row = RowTemplate { id : & item . id , completed : item . completed , title : & item . title , } ; if let Ok (res) = row . render () { output . push_str (& res) ; } } output } # [doc = ""] # [doc = " Format the contents of an \"items left\" indicator."] # [doc = ""] # [doc = " `active_todos` Number of active todos"] # [doc = ""] # [doc = " Returns the contents for an \"items left\" indicator"] pub fn item_counter (active_todos : usize) -> String { let items_left = ItemsLeftTemplate { active_todos } ; items_left . render () . unwrap_or_default () } }
};
}

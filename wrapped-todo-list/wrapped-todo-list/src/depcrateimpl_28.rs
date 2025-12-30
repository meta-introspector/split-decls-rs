// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl From < & TodoItem > for ListItem < '_ > { fn from (value : & TodoItem) -> Self { let line = match value . status { Status :: Todo => Line :: styled (format ! (" ☐ {}" , value . todo) , TEXT_FG_COLOR) , Status :: Completed => { Line :: styled (format ! (" ✓ {}" , value . todo) , COMPLETED_TEXT_FG_COLOR) } } ; ListItem :: new (line) } }
};
}

// Generated macro for last_item_shape (function)
macro_rules! Depcrate_overflowlast_item_shape {
() => {
// Module: crate::overflow
// Provides: {"last_item_shape"}
// Dependencies: {}
# [doc = " Returns a shape for the last argument which is going to be overflowed."] fn last_item_shape (lists : & [OverflowableItem < '_ >] , items : & [ListItem] , shape : Shape , args_max_width : usize ,) -> Option < Shape > { if items . len () == 1 && ! lists . get (0) ? . is_nested_call () { return Some (shape) ; } let offset = items . iter () . dropping_back (1) . map (| i | { 2 + i . inner_as_ref () . len () }) . sum () ; Shape { width : min (args_max_width , shape . width) , .. shape } . offset_left_opt (offset) }
};
}

// Generated macro for wrap_reorderable_items (function)
macro_rules! Depcrate_reorderwrap_reorderable_items {
() => {
// Module: crate::reorder
// Provides: {"wrap_reorderable_items"}
// Dependencies: {}
fn wrap_reorderable_items (context : & RewriteContext < '_ > , list_items : & [ListItem] , shape : Shape ,) -> RewriteResult { let fmt = ListFormatting :: new (shape , context . config) . separator ("") . align_comments (false) ; write_list (list_items , & fmt) }
};
}

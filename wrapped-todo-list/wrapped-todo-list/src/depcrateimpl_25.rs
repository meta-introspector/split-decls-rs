// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl Widget for & mut App { fn render (self , area : Rect , buf : & mut Buffer) { let main_layout = Layout :: vertical ([Constraint :: Length (2) , Constraint :: Fill (1) , Constraint :: Length (1) ,]) ; let [header_area , content_area , footer_area] = area . layout (& main_layout) ; let content_layout = Layout :: vertical ([Constraint :: Fill (1) , Constraint :: Fill (1)]) ; let [list_area , item_area] = content_area . layout (& content_layout) ; App :: render_header (header_area , buf) ; App :: render_footer (footer_area , buf) ; self . render_list (list_area , buf) ; self . render_selected_item (item_area , buf) ; } }
};
}

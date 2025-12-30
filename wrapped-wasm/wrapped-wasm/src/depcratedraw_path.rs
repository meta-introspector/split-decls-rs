// Generated macro for draw_path (function)
macro_rules! Depcratedraw_path {
() => {
// Module: crate
// Provides: {"draw_path"}
// Dependencies: {}
fn draw_path (ctx : & web_sys :: CanvasRenderingContext2d , style : & str , (mut x , mut y) : (f64 , f64) , steps : & [(f64 , f64)] ,) { ctx . save () ; ctx . set_stroke_style_str (style) ; ctx . begin_path () ; ctx . move_to (x , y) ; for (delta_x , delta_y) in steps { x += delta_x ; y += delta_y ; ctx . line_to (x , y) ; } ctx . stroke () ; ctx . restore () ; }
};
}

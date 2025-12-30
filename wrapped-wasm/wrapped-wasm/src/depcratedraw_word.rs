// Generated macro for draw_word (function)
macro_rules! Depcratedraw_word {
() => {
// Module: crate
// Provides: {"draw_word"}
// Dependencies: {}
fn draw_word (ctx : & web_sys :: CanvasRenderingContext2d , x : f64 , y : f64 , word : & CanvasWord , last_word : bool ,) -> Result < () , JsValue > { ctx . fill_text (word . word , x , y) ? ; draw_path (ctx , "orange" , (x , y - 10.0) , & [(0.0 , 10.0) , (word . width , 0.0)] ,) ; ctx . save () ; ctx . set_font ("10px sans-serif") ; ctx . set_text_align ("center") ; ctx . set_text_baseline ("top") ; ctx . fill_text (& format ! ("{:.1}px" , word . width) , x + word . width / 2.0 , y + 3.0 ,) ? ; ctx . restore () ; let x = x + word . width ; if last_word { ctx . fill_text (word . penalty , x , y) ? ; draw_path (ctx , "red" , (x , y) , & [(word . penalty_width , 0.0)]) ; } else { ctx . fill_text (word . whitespace , x , y) ? ; draw_path (ctx , "lightblue" , (x , y) , & [(word . whitespace_width , 0.0)]) ; } Ok (()) }
};
}

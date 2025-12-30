// Generated macro for canvas_width (function)
macro_rules! Depcratecanvas_width {
() => {
// Module: crate
// Provides: {"canvas_width"}
// Dependencies: {}
fn canvas_width (ctx : & web_sys :: CanvasRenderingContext2d , text : & str) -> f64 { ctx . measure_text (text) . unwrap () . width () }
};
}

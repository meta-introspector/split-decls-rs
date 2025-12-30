// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a > CanvasWord < 'a > { fn from (ctx : & '_ web_sys :: CanvasRenderingContext2d , word : textwrap :: core :: Word < 'a >) -> Self { CanvasWord { word : word . word , width : canvas_width (ctx , word . word) , whitespace : word . whitespace , whitespace_width : canvas_width (ctx , word . whitespace) , penalty : word . penalty , penalty_width : canvas_width (ctx , word . penalty) , } } fn break_apart (self , ctx : & '_ web_sys :: CanvasRenderingContext2d , max_width : f64 ,) -> Vec < CanvasWord < 'a > > { if self . width <= max_width { return vec ! [self] ; } let mut start = 0 ; let mut words = Vec :: new () ; for (idx , grapheme) in self . word . grapheme_indices (true) { let with_grapheme = & self . word [start .. idx + grapheme . len ()] ; let without_grapheme = & self . word [start .. idx] ; if idx > 0 && canvas_width (ctx , with_grapheme) > max_width { let natural_width = canvas_width (ctx , without_grapheme) ; words . push (CanvasWord { word : without_grapheme , width : max_width . max (natural_width) , whitespace : "" , whitespace_width : 0.0 , penalty : "" , penalty_width : 0.0 , }) ; start = idx ; } } words . push (CanvasWord { word : & self . word [start ..] , width : canvas_width (ctx , & self . word [start ..]) , whitespace : self . whitespace , whitespace_width : self . whitespace_width , penalty : self . penalty , penalty_width : self . penalty_width , }) ; words } }
};
}

// Generated macro for ops (macro)
macro_rules! Depcrate_rangeops {
() => {
// Module: crate::range
// Provides: {"ops"}
// Dependencies: {}
macro_rules ! ops { (impl $ Op : ident for TextRange by fn $ f : ident = $ op : tt) => { impl $ Op <& TextSize > for TextRange { type Output = TextRange ; # [inline] fn $ f (self , other : & TextSize) -> TextRange { self $ op * other } } impl < T > $ Op < T > for & TextRange where TextRange : $ Op < T , Output = TextRange >, { type Output = TextRange ; # [inline] fn $ f (self , other : T) -> TextRange { * self $ op other } } } ; }
};
}

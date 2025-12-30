// Generated macro for ops (macro)
macro_rules! Depcrate_sizeops {
() => {
// Module: crate::size
// Provides: {"ops"}
// Dependencies: {}
macro_rules ! ops { (impl $ Op : ident for TextSize by fn $ f : ident = $ op : tt) => { impl $ Op < TextSize > for TextSize { type Output = TextSize ; # [inline] fn $ f (self , other : TextSize) -> TextSize { TextSize { raw : self . raw $ op other . raw } } } impl $ Op <& TextSize > for TextSize { type Output = TextSize ; # [inline] fn $ f (self , other : & TextSize) -> TextSize { self $ op * other } } impl < T > $ Op < T > for & TextSize where TextSize : $ Op < T , Output = TextSize >, { type Output = TextSize ; # [inline] fn $ f (self , other : T) -> TextSize { * self $ op other } } } ; }
};
}

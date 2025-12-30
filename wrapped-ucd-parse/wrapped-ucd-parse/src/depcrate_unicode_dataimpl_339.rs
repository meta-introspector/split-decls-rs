// Generated macro for impl_339 (impl)
macro_rules! Depcrate_unicode_dataimpl_339 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_339"}
// Dependencies: {}
impl < I : Iterator < Item = UnicodeData > > UnicodeDataExpander < I > { # [doc = " Create a new iterator that expands pairs of `UnicodeData` range"] # [doc = " records. All other records are passed through as-is."] pub fn new < T > (it : T) -> UnicodeDataExpander < I > where T : IntoIterator < IntoIter = I , Item = I :: Item > , { UnicodeDataExpander { it : it . into_iter () . peekable () , range : CodepointRange { range : 0 .. 0 , start_record : UnicodeData :: default () , } , } } }
};
}

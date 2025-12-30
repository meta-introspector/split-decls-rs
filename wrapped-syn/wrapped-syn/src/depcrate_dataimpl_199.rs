// Generated macro for impl_199 (impl)
macro_rules! Depcrate_dataimpl_199 {
() => {
// Module: crate::data
// Provides: {"impl_199"}
// Dependencies: {}
impl IntoIterator for Fields { type Item = Field ; type IntoIter = punctuated :: IntoIter < Field > ; fn into_iter (self) -> Self :: IntoIter { match self { Fields :: Unit => Punctuated :: < Field , () > :: new () . into_iter () , Fields :: Named (f) => f . named . into_iter () , Fields :: Unnamed (f) => f . unnamed . into_iter () , } } }
};
}

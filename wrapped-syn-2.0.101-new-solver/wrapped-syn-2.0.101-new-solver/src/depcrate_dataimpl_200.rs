// Generated macro for impl_200 (impl)
macro_rules! Depcrate_dataimpl_200 {
() => {
// Module: crate::data
// Provides: {"impl_200"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Fields { type Item = & 'a mut Field ; type IntoIter = punctuated :: IterMut < 'a , Field > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}

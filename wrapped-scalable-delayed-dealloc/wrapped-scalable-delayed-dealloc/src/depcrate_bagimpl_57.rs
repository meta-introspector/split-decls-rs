// Generated macro for impl_57 (impl)
macro_rules! Depcrate_bagimpl_57 {
() => {
// Module: crate::bag
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'b , T , const ARRAY_LEN : usize > IntoIterator for & 'b mut Bag < T , ARRAY_LEN > { type IntoIter = IterMut < 'b , T , ARRAY_LEN > ; type Item = & 'b mut T ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}

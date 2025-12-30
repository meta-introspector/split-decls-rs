// Generated macro for IterMutTrait (trait)
macro_rules! Depcrate_punctuatedIterMutTrait {
() => {
// Module: crate::punctuated
// Provides: {"IterMutTrait"}
// Dependencies: {}
trait IterMutTrait < 'a , T : 'a > : DoubleEndedIterator < Item = & 'a mut T > + ExactSizeIterator < Item = & 'a mut T > { }
};
}

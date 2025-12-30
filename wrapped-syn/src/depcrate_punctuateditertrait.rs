// Generated macro for IterTrait (trait)
macro_rules! Depcrate_punctuatedIterTrait {
() => {
// Module: crate::punctuated
// Provides: {"IterTrait"}
// Dependencies: {}
trait IterTrait < 'a , T : 'a > : Iterator < Item = & 'a T > + DoubleEndedIterator + ExactSizeIterator { fn clone_box (& self) -> Box < NoDrop < dyn IterTrait < 'a , T > + 'a > > ; }
};
}

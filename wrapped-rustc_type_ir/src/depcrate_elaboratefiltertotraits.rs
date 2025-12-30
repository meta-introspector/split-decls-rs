// Generated macro for FilterToTraits (struct)
macro_rules! Depcrate_elaborateFilterToTraits {
() => {
// Module: crate::elaborate
// Provides: {"FilterToTraits"}
// Dependencies: {}
# [doc = " A filter around an iterator of predicates that makes it yield up"] # [doc = " just trait references."] pub struct FilterToTraits < I : Interner , It : Iterator < Item = I :: Clause > > { _cx : PhantomData < I > , base_iterator : It , }
};
}

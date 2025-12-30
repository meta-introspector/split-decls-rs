// Generated macro for impl_259 (impl)
macro_rules! Depcrate_commentimpl_259 {
() => {
// Module: crate::comment
// Provides: {"impl_259"}
// Dependencies: {}
impl < T > CharClasses < T > where T : Iterator , T :: Item : RichChar , { pub (crate) fn new (base : T) -> CharClasses < T > { CharClasses { base : multipeek (base) , status : CharClassesStatus :: Normal , } } }
};
}

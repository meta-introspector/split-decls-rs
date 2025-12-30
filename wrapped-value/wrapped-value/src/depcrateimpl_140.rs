// Generated macro for impl_140 (impl)
macro_rules! Depcrateimpl_140 {
() => {
// Module: crate
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'a , T : Clone + Into < ConstValue > > From < & 'a [T] > for ConstValue { fn from (f : & 'a [T]) -> Self { ConstValue :: List (f . iter () . cloned () . map (Into :: into) . collect ()) } }
};
}

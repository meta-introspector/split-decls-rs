// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl FromIterator < (Status , & 'static str , & 'static str) > for TodoList { fn from_iter < I : IntoIterator < Item = (Status , & 'static str , & 'static str) > > (iter : I) -> Self { let items = iter . into_iter () . map (| (status , todo , info) | TodoItem :: new (status , todo , info)) . collect () ; let state = ListState :: default () ; Self { items , state } } }
};
}

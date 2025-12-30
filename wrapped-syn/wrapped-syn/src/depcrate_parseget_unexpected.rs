// Generated macro for get_unexpected (function)
macro_rules! Depcrate_parseget_unexpected {
() => {
// Module: crate::parse
// Provides: {"get_unexpected"}
// Dependencies: {}
pub (crate) fn get_unexpected (buffer : & ParseBuffer) -> Rc < Cell < Unexpected > > { cell_clone (& buffer . unexpected) . unwrap () }
};
}

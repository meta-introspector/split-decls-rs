// Generated macro for new (function)
macro_rules! Depcrate_spec_crt_objectsnew {
() => {
// Module: crate::spec::crt_objects
// Provides: {"new"}
// Dependencies: {}
pub (super) fn new (obj_table : & [(LinkOutputKind , & [& 'static str])]) -> CrtObjects { obj_table . iter () . map (| (z , k) | (* z , k . iter () . map (| b | (* b) . into ()) . collect ())) . collect () }
};
}

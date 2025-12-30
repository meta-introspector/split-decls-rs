// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl SymInfo { fn new (sym : & Symbol , obj : & ObjFile , obj_path : & str) -> Self { let section = sym . section () ; let section_name = sym . section () . index () . and_then (| idx | obj . section_by_index (idx) . ok ()) . and_then (| sec | sec . name () . ok ()) . map (ToString :: to_string) . unwrap_or_else (| | format ! ("{section:?}")) ; Self { name : sym . name () . expect ("missing name") . to_owned () , kind : sym . kind () , scope : sym . scope () , section : section_name , is_undefined : sym . is_undefined () , is_global : sym . is_global () , is_local : sym . is_local () , is_weak : sym . is_weak () , is_common : sym . is_common () , address : sym . address () , object : obj_path . to_owned () , } } }
};
}

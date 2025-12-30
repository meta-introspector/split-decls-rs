// Generated macro for impl_743 (impl)
macro_rules! Depcrate_unix_linux_componentimpl_743 {
() => {
// Module: crate::unix::linux::component
// Provides: {"impl_743"}
// Dependencies: {}
impl ComponentsInner { pub (crate) fn new () -> Self { Self { components : Vec :: with_capacity (4) , } } pub (crate) fn from_vec (components : Vec < Component >) -> Self { Self { components } } pub (crate) fn into_vec (self) -> Vec < Component > { self . components } pub (crate) fn list (& self) -> & [Component] { & self . components } pub (crate) fn list_mut (& mut self) -> & mut [Component] { & mut self . components } pub (crate) fn refresh (& mut self) { self . refresh_from_sys_class_path (Path :: new ("/sys/class")) ; } fn refresh_from_sys_class_path (& mut self , path : & Path) { read_temp_dir (& path . join ("hwmon") , "hwmon" , | path | { ComponentInner :: from_hwmon (& mut self . components , & path) ; }) ; if self . components . is_empty () { read_temp_dir (& path . join ("thermal") , "thermal_" , | path | { let temp = path . join ("temp") ; if temp . exists () { let Some (name) = get_file_line (& path . join ("type") , 16) else { return ; } ; let component_id = path . file_name () . and_then (OsStr :: to_str) . map (str :: to_string) ; let mut component = ComponentInner { name , id : component_id , .. Default :: default () } ; fill_component (& mut component , "input" , & path , "temp") ; self . components . push (Component { inner : component }) ; } }) ; } } }
};
}

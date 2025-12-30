// Generated macro for impl_400 (impl)
macro_rules! Depcrate_unix_apple_macos_component_x86impl_400 {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"impl_400"}
// Dependencies: {}
impl ComponentsInner { pub (crate) fn new () -> Self { Self { components : Vec :: with_capacity (2) , connection : IoService :: new_connection () , } } pub (crate) fn from_vec (components : Vec < Component >) -> Self { Self { components , connection : IoService :: new_connection () , } } pub (crate) fn into_vec (self) -> Vec < Component > { self . components } pub (crate) fn list (& self) -> & [Component] { & self . components } pub (crate) fn list_mut (& mut self) -> & mut [Component] { & mut self . components } pub (crate) fn refresh (& mut self) { let Some (ref connection) = self . connection else { sysinfo_debug ! ("No connection to IoService, skipping components refresh") ; return ; } ; let connection = connection . inner () ; let critical_temp = get_temperature (connection , & ['T' as i8 , 'C' as i8 , '0' as i8 , 'D' as i8 , 0]) ; for (label , id , v) in COMPONENTS_TEMPERATURE_IDS . iter () { if let Some (c) = self . components . iter_mut () . find (| c | c . inner . id == * id) { c . refresh () ; c . inner . updated = true ; } else if let Some (c) = ComponentInner :: new ((* id) . to_owned () , (* label) . to_owned () , None , critical_temp , v , connection ,) { self . components . push (Component { inner : c }) ; } } } }
};
}

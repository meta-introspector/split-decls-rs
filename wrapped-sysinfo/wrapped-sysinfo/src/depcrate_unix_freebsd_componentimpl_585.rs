// Generated macro for impl_585 (impl)
macro_rules! Depcrate_unix_freebsd_componentimpl_585 {
() => {
// Module: crate::unix::freebsd::component
// Provides: {"impl_585"}
// Dependencies: {}
impl ComponentsInner { pub (crate) fn new () -> Self { let nb_cpus = unsafe { super :: utils :: get_nb_cpus () } ; Self { nb_cpus , components : Vec :: with_capacity (nb_cpus) , } } pub (crate) fn from_vec (components : Vec < Component >) -> Self { Self { nb_cpus : unsafe { super :: utils :: get_nb_cpus () } , components , } } pub (crate) fn into_vec (self) -> Vec < Component > { self . components } pub (crate) fn list (& self) -> & [Component] { & self . components } pub (crate) fn list_mut (& mut self) -> & mut [Component] { & mut self . components } pub (crate) fn refresh (& mut self) { if self . components . len () != self . nb_cpus { for core in 0 .. self . nb_cpus { unsafe { let id = format ! ("dev.cpu.{core}.temperature\0") . as_bytes () . to_vec () ; if let Some (temperature) = refresh_component (& id) { self . components . push (Component { inner : ComponentInner :: new (id , temperature , core) , }) ; } } } } else { for c in self . components . iter_mut () { c . refresh () ; c . inner . updated = true ; } } } }
};
}

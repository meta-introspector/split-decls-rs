// Generated macro for impl_582 (impl)
macro_rules! Depcrate_unix_freebsd_componentimpl_582 {
() => {
// Module: crate::unix::freebsd::component
// Provides: {"impl_582"}
// Dependencies: {}
impl ComponentInner { pub (crate) fn new (id : Vec < u8 > , temperature : f32 , core : usize) -> ComponentInner { ComponentInner { id , component_id : format ! ("cpu_{}" , core + 1) , label : format ! ("CPU {}" , core + 1) , temperature : Some (temperature) , max : temperature , updated : true , } } pub (crate) fn temperature (& self) -> Option < f32 > { self . temperature } pub (crate) fn max (& self) -> Option < f32 > { Some (self . max) } pub (crate) fn critical (& self) -> Option < f32 > { None } pub (crate) fn id (& self) -> Option < & str > { Some (& self . component_id) } pub (crate) fn label (& self) -> & str { & self . label } pub (crate) fn refresh (& mut self) { unsafe { self . temperature = refresh_component (& self . id) ; if let Some (temperature) = self . temperature && temperature > self . max { self . max = temperature ; } } } }
};
}

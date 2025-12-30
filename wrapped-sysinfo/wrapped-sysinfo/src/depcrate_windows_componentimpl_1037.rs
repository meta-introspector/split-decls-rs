// Generated macro for impl_1037 (impl)
macro_rules! Depcrate_windows_componentimpl_1037 {
() => {
// Module: crate::windows::component
// Provides: {"impl_1037"}
// Dependencies: {}
impl ComponentInner { # [doc = " Creates a new `ComponentInner` with the given information."] fn new () -> Option < Self > { let mut c = Connection :: new () . and_then (| x | x . create_instance ()) . and_then (| x | x . connect_server ()) . and_then (| x | x . set_proxy_blanket ()) . and_then (| x | x . exec_query ()) ? ; c . temperature (true) . map (| (temperature , critical) | ComponentInner { temperature , label : "Computer" . to_owned () , max : temperature , critical , connection : Some (c) , updated : true , }) } pub (crate) fn temperature (& self) -> Option < f32 > { Some (self . temperature) } pub (crate) fn max (& self) -> Option < f32 > { Some (self . max) } pub (crate) fn critical (& self) -> Option < f32 > { self . critical } pub (crate) fn label (& self) -> & str { & self . label } pub (crate) fn id (& self) -> Option < & str > { Some (& self . label) } pub (crate) fn refresh (& mut self) { if self . connection . is_none () { self . connection = Connection :: new () . and_then (| x | x . create_instance ()) . and_then (| x | x . connect_server ()) . and_then (| x | x . set_proxy_blanket ()) ; } self . connection = if let Some (x) = self . connection . take () { x . exec_query () } else { None } ; if let Some (ref mut connection) = self . connection && let Some ((temperature , _)) = connection . temperature (false) { self . temperature = temperature ; if self . temperature > self . max { self . max = self . temperature ; } } } }
};
}

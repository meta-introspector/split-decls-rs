// Generated macro for impl_93 (impl)
macro_rules! Depcrate_yamlimpl_93 {
() => {
// Module: crate::yaml
// Provides: {"impl_93"}
// Dependencies: {}
impl MarkedEventReceiver for YamlLoader { fn on_event (& mut self , ev : Event , mark : Marker) { if self . error . is_some () { return ; } if let Err (e) = self . on_event_impl (ev , mark) { self . error = Some (e) ; } } }
};
}

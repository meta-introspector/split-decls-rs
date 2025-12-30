// Generated macro for UnsyncBoxService (struct)
macro_rules! Depcrate_util_boxed_unsyncUnsyncBoxService {
() => {
// Module: crate::util::boxed::unsync
// Provides: {"UnsyncBoxService"}
// Dependencies: {}
# [doc = " A boxed [`Service`] trait object."] pub struct UnsyncBoxService < T , U , E > { inner : Box < dyn Service < T , Response = U , Error = E , Future = UnsyncBoxFuture < U , E > > > , }
};
}

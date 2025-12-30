// Generated macro for CloneService (trait)
macro_rules! Depcrate_util_boxed_clone_syncCloneService {
() => {
// Module: crate::util::boxed_clone_sync
// Provides: {"CloneService"}
// Dependencies: {}
trait CloneService < R > : Service < R > { fn clone_box (& self ,) -> Box < dyn CloneService < R , Response = Self :: Response , Error = Self :: Error , Future = Self :: Future > + Send + Sync , > ; }
};
}

// Generated macro for BoxCloneSyncService (struct)
macro_rules! Depcrate_util_boxed_clone_syncBoxCloneSyncService {
() => {
// Module: crate::util::boxed_clone_sync
// Provides: {"BoxCloneSyncService"}
// Dependencies: {}
# [doc = " A [`Clone`] + [`Send`] + [`Sync`] boxed [`Service`]."] # [doc = ""] # [doc = " [`BoxCloneSyncService`] turns a service into a trait object, allowing the"] # [doc = " response future type to be dynamic, and allowing the service to be cloned and shared."] # [doc = ""] # [doc = " This is similar to [`BoxCloneService`](super::BoxCloneService) except the resulting"] # [doc = " service implements [`Sync`]."] pub struct BoxCloneSyncService < T , U , E > (Box < dyn CloneService < T , Response = U , Error = E , Future = BoxFuture < 'static , Result < U , E > > > + Send + Sync , > ,) ;
};
}

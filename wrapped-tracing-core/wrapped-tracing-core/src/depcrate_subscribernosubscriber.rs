// Generated macro for NoSubscriber (struct)
macro_rules! Depcrate_subscriberNoSubscriber {
() => {
// Module: crate::subscriber
// Provides: {"NoSubscriber"}
// Dependencies: {}
# [doc = " A no-op [`Subscriber`]."] # [doc = ""] # [doc = " [`NoSubscriber`] implements the [`Subscriber`] trait by never being enabled,"] # [doc = " never being interested in any callsite, and dropping all spans and events."] # [derive (Copy , Clone , Debug , Default)] pub struct NoSubscriber (()) ;
};
}

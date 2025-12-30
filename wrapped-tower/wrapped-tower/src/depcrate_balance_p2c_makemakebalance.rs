// Generated macro for MakeBalance (struct)
macro_rules! Depcrate_balance_p2c_makeMakeBalance {
() => {
// Module: crate::balance::p2c::make
// Provides: {"MakeBalance"}
// Dependencies: {}
# [doc = " Constructs load balancers over dynamic service sets produced by a wrapped \"inner\" service."] # [doc = ""] # [doc = " This is effectively an implementation of [`MakeService`] except that it forwards the service"] # [doc = " descriptors (`Target`) to an inner service (`S`), and expects that service to produce a"] # [doc = " service set in the form of a [`Discover`]. It then wraps the service set in a [`Balance`]"] # [doc = " before returning it as the \"made\" service."] # [doc = ""] # [doc = " See the [module-level documentation](crate::balance) for details on load balancing."] # [doc = ""] # [doc = " [`MakeService`]: crate::MakeService"] # [doc = " [`Discover`]: crate::discover::Discover"] # [doc = " [`Balance`]: crate::balance::p2c::Balance"] pub struct MakeBalance < S , Req > { inner : S , _marker : PhantomData < fn (Req) > , }
};
}

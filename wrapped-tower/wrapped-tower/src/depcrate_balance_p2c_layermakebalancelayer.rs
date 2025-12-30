// Generated macro for MakeBalanceLayer (struct)
macro_rules! Depcrate_balance_p2c_layerMakeBalanceLayer {
() => {
// Module: crate::balance::p2c::layer
// Provides: {"MakeBalanceLayer"}
// Dependencies: {}
# [doc = " Construct load balancers ([`Balance`]) over dynamic service sets ([`Discover`]) produced by the"] # [doc = " \"inner\" service in response to requests coming from the \"outer\" service."] # [doc = ""] # [doc = " This construction may seem a little odd at first glance. This is not a layer that takes"] # [doc = " requests and produces responses in the traditional sense. Instead, it is more like"] # [doc = " [`MakeService`] in that it takes service _descriptors_ (see `Target` on [`MakeService`])"] # [doc = " and produces _services_. Since [`Balance`] spreads requests across a _set_ of services,"] # [doc = " the inner service should produce a [`Discover`], not just a single"] # [doc = " [`Service`], given a service descriptor."] # [doc = ""] # [doc = " See the [module-level documentation](crate::balance) for details on load balancing."] # [doc = ""] # [doc = " [`Balance`]: crate::balance::p2c::Balance"] # [doc = " [`Discover`]: crate::discover::Discover"] # [doc = " [`MakeService`]: crate::MakeService"] # [doc = " [`Service`]: crate::Service"] pub struct MakeBalanceLayer < D , Req > { _marker : PhantomData < fn (D , Req) > , }
};
}

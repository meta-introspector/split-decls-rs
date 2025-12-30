// Generated macro for Balance (struct)
macro_rules! Depcrate_balance_p2c_serviceBalance {
() => {
// Module: crate::balance::p2c::service
// Provides: {"Balance"}
// Dependencies: {}
# [doc = " Efficiently distributes requests across an arbitrary number of services."] # [doc = ""] # [doc = " See the [module-level documentation](..) for details."] # [doc = ""] # [doc = " Note that [`Balance`] requires that the [`Discover`] you use is [`Unpin`] in order to implement"] # [doc = " [`Service`]. This is because it needs to be accessed from [`Service::poll_ready`], which takes"] # [doc = " `&mut self`. You can achieve this easily by wrapping your [`Discover`] in [`Box::pin`] before you"] # [doc = " construct the [`Balance`] instance. For more details, see [#319]."] # [doc = ""] # [doc = " [`Box::pin`]: std::boxed::Box::pin()"] # [doc = " [#319]: https://github.com/tower-rs/tower/issues/319"] pub struct Balance < D , Req > where D : Discover , D :: Key : Hash , { discover : D , services : ReadyCache < D :: Key , D :: Service , Req > , ready_index : Option < usize > , rng : Box < dyn Rng + Send + Sync > , _req : PhantomData < Req > , }
};
}

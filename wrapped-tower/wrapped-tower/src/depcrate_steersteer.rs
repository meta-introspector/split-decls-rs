// Generated macro for Steer (struct)
macro_rules! Depcrate_steerSteer {
() => {
// Module: crate::steer
// Provides: {"Steer"}
// Dependencies: {}
# [doc = " [`Steer`] manages a list of [`Service`]s which all handle the same type of request."] # [doc = ""] # [doc = " An example use case is a sharded service."] # [doc = " It accepts new requests, then:"] # [doc = " 1. Determines, via the provided [`Picker`], which [`Service`] the request corresponds to."] # [doc = " 2. Waits (in [`Service::poll_ready`]) for *all* services to be ready."] # [doc = " 3. Calls the correct [`Service`] with the request, and returns a future corresponding to the"] # [doc = "    call."] # [doc = ""] # [doc = " Note that [`Steer`] must wait for all services to be ready since it can't know ahead of time"] # [doc = " which [`Service`] the next message will arrive for, and is unwilling to buffer items"] # [doc = " indefinitely. This will cause head-of-line blocking unless paired with a [`Service`] that does"] # [doc = " buffer items indefinitely, and thus always returns [`Poll::Ready`]. For example, wrapping each"] # [doc = " component service with a [`Buffer`] with a high enough limit (the maximum number of concurrent"] # [doc = " requests) will prevent head-of-line blocking in [`Steer`]."] # [doc = ""] # [doc = " [`Buffer`]: crate::buffer::Buffer"] pub struct Steer < S , F , Req > { router : F , services : Vec < S > , not_ready : VecDeque < usize > , _phantom : PhantomData < Req > , }
};
}

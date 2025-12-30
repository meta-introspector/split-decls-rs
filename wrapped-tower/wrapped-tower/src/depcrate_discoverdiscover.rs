// Generated macro for Discover (trait)
macro_rules! Depcrate_discoverDiscover {
() => {
// Module: crate::discover
// Provides: {"Discover"}
// Dependencies: {}
# [doc = " A dynamically changing set of related services."] # [doc = ""] # [doc = " As new services arrive and old services are retired,"] # [doc = " [`Change`]s are returned which provide unique identifiers"] # [doc = " for the services."] # [doc = ""] # [doc = " See the module documentation for more details."] pub trait Discover : Sealed < Change < () , () > > { # [doc = " A unique identifier for each active service."] # [doc = ""] # [doc = " An identifier can be re-used once a [`Change::Remove`] has been yielded for its service."] type Key : Eq ; # [doc = " The type of [`Service`] yielded by this [`Discover`]."] # [doc = ""] # [doc = " [`Service`]: crate::Service"] type Service ; # [doc = " Error produced during discovery"] type Error ; # [doc = " Yields the next discovery change set."] fn poll_discover (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Change < Self :: Key , Self :: Service > , Self :: Error > > > ; }
};
}

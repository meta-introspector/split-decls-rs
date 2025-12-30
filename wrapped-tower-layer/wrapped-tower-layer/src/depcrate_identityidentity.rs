// Generated macro for Identity (struct)
macro_rules! Depcrate_identityIdentity {
() => {
// Module: crate::identity
// Provides: {"Identity"}
// Dependencies: {}
# [doc = " A no-op middleware."] # [doc = ""] # [doc = " When wrapping a [`Service`], the [`Identity`] layer returns the provided"] # [doc = " service without modifying it."] # [doc = ""] # [doc = " [`Service`]: https://docs.rs/tower-service/latest/tower_service/trait.Service.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use tower_layer::Identity;"] # [doc = " use tower_layer::Layer;"] # [doc = ""] # [doc = " let identity = Identity::new();"] # [doc = ""] # [doc = " assert_eq!(identity.layer(42), 42);"] # [doc = " ```"] # [derive (Default , Clone)] pub struct Identity { _p : () , }
};
}
